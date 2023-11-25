use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    fetcher::{FetchError, Fetcher},
    item::{Item, Name, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
    LoadError, ReadError,
};
use futures::FutureExt;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, error::SendError},
    oneshot::{self, Sender},
};
use tracing::warn;
use url::Url;

use std::{error::Error, str::FromStr};

use crate::{
    json::{
        migrate::{groceries, migrate_recipes, migrate_sections},
        JsonStore,
    },
    sqlite::{DbUri, SqliteStore},
};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("SQLite database connection error: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("Connection pool error: {0}")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),

    #[error("invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("fetch error: {0}")]
    FetchError(#[from] FetchError),

    #[error("JoinError: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("load error: {0}")]
    LoadError(#[from] LoadError),

    #[error("migration error: {0}")]
    MigrationError(#[from] Box<dyn Error + Send + Sync>),

    #[error("Parse store type error: {0}")]
    ParseStoreType(String),

    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),

    #[error("ingredients not found for: {0}")]
    RecipeIngredients(String),

    #[error("ingredients not found for: {0}")]
    SendError(#[from] SendError<(ApiCommand, Sender<Result<StoreResponse, StoreError>>)>),
}

#[derive(Debug)]
pub enum StoreType {
    Json,
    Sqlite,
    SqliteInmem,
}

impl FromStr for StoreType {
    type Err = StoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "sqlite" => Ok(Self::Sqlite),
            _ => Err(StoreError::ParseStoreType(
                "Store types are currently limited to 'sqlite' and 'json'.".to_string(),
            )),
        }
    }
}

#[derive(Clone)]
pub enum Store {
    Json(JsonStore),
    Sqlite(SqliteStore),
}

impl From<SqliteStore> for Store {
    fn from(store: SqliteStore) -> Self {
        Self::Sqlite(store)
    }
}

impl From<JsonStore> for Store {
    fn from(store: JsonStore) -> Self {
        Self::Json(store)
    }
}

#[derive(Debug)]
pub enum StoreResponse {
    AddedItem(Name),
    AddedListItem(Name),
    AddedListRecipe(Recipe),
    AddedRecipe(Recipe),
    Checklist(Vec<Item>),
    DeletedRecipe(Recipe),
    DeletedChecklistItem(Name),
    FetchedRecipe((Recipe, Ingredients)),
    Items(Items),
    JsonToSqlite,
    List(List),
    NothingReturned(ApiCommand),
    Recipes(Vec<Recipe>),
    RecipeIngredients(Ingredients),
    RefreshList,
    Sections(Vec<Section>),
}

#[derive(Clone)]
pub struct StoreDispatch {
    tx: mpsc::Sender<(
        ApiCommand,
        oneshot::Sender<Result<StoreResponse, StoreError>>,
    )>,
}

impl StoreDispatch {
    pub async fn send(
        &self,
        msg: (
            ApiCommand,
            oneshot::Sender<Result<StoreResponse, StoreError>>,
        ),
    ) -> Result<(), StoreError> {
        self.tx.send(msg).await?;
        Ok(())
    }
}

impl Store {
    pub async fn init(store_type: StoreType) -> Result<StoreDispatch, StoreError> {
        let mut store = match store_type {
            StoreType::Json => Store::from(JsonStore::default()),
            StoreType::Sqlite => Store::from(SqliteStore::new(DbUri::new()).await?),
            StoreType::SqliteInmem => Store::from(SqliteStore::new(DbUri::inmem()).await?),
        };

        let (tx, mut rx) = mpsc::channel::<(
            ApiCommand,
            oneshot::Sender<Result<StoreResponse, StoreError>>,
        )>(10);

        tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    cmd = rx.recv().fuse() => {
                        if let Some((command, reply)) = cmd {
                            let result = store
                                .execute_transaction(command)
                                .await;

                            reply
                                .send(result)
                                .map_err(|e| {
                                    warn!(?e, "Send reply to API consumer failed");
                                })
                                .ok();
                        }
                    }
                    else => break
                }
            }
        });

        Ok(StoreDispatch { tx })
    }

    async fn execute_transaction(
        &mut self,
        command: ApiCommand,
    ) -> Result<StoreResponse, StoreError> {
        match command {
            ApiCommand::Add(cmd) => self.add(cmd).await,
            ApiCommand::Delete(cmd) => self.delete(cmd).await,
            ApiCommand::FetchRecipe(url) => self.fetch_recipe(url).await,
            ApiCommand::MigrateJsonDbToSqlite => self.migrate_json_store_to_sqlite().await,
            ApiCommand::Read(cmd) => self.read(cmd).await,
            ApiCommand::Update(cmd) => self.update(cmd).await,
        }
    }

    async fn add(&mut self, cmd: Add) -> Result<StoreResponse, StoreError> {
        match cmd {
            Add::ChecklistItem(name) => {
                self.add_checklist_item(&name).await?;
                Ok(StoreResponse::AddedItem(name))
            }
            Add::Item { name, .. } => {
                self.add_item(&name).await?;
                Ok(StoreResponse::AddedItem(name))
            }
            Add::ListItem(name) => {
                self.add_list_item(&name).await?;
                Ok(StoreResponse::AddedListItem(name))
            }
            Add::ListRecipe(name) => {
                self.add_list_recipe(&name).await?;
                Ok(StoreResponse::AddedListRecipe(name))
            }
            Add::Recipe {
                recipe,
                ingredients,
            } => {
                self.add_recipe(&recipe, &ingredients).await?;
                Ok(StoreResponse::AddedRecipe(recipe))
            }
        }
    }

    async fn read(&mut self, cmd: Read) -> Result<StoreResponse, StoreError> {
        match cmd {
            Read::All => {
                let results = self.items().await?;
                Ok(StoreResponse::Items(results))
            }
            Read::Checklist => {
                let items = self.checklist().await?;
                Ok(StoreResponse::Checklist(items))
            }
            Read::Item(_name) => todo!(),
            Read::List => {
                let list = self.list().await?;
                Ok(StoreResponse::List(list))
            }
            Read::ListRecipes => todo!(),
            Read::Recipe(recipe) => match self.recipe_ingredients(&recipe).await {
                Ok(Some(ingredients)) => Ok(StoreResponse::RecipeIngredients(ingredients)),
                Ok(None) => Ok(StoreResponse::NothingReturned(ApiCommand::Read(
                    Read::Recipe(recipe),
                ))),
                Err(e) => Err(e),
            },
            Read::Recipes => Ok(StoreResponse::Recipes(self.recipes().await?)),
            Read::Sections => {
                let results = self.sections().await?;
                Ok(StoreResponse::Sections(results))
            }
        }
    }

    async fn update(&mut self, cmd: Update) -> Result<StoreResponse, StoreError> {
        match cmd {
            Update::Item(_name) => todo!(),
            Update::RefreshList => {
                self.refresh_list().await?;
                Ok(StoreResponse::RefreshList)
            }
            Update::Recipe(_name) => todo!(),
        }
    }

    async fn delete(&mut self, cmd: Delete) -> Result<StoreResponse, StoreError> {
        match cmd {
            Delete::ChecklistItem(name) => {
                self.delete_checklist_item(&name).await?;
                Ok(StoreResponse::DeletedChecklistItem(name))
            }
            Delete::ClearChecklist => todo!(),
            Delete::ClearList => todo!(),
            Delete::Item(_name) => todo!(),
            Delete::ListItem(_name) => todo!(),
            Delete::Recipe(recipe) => {
                let recipe = self.delete_recipe(&recipe).await?;
                Ok(StoreResponse::DeletedRecipe(recipe))
            }
        }
    }

    async fn fetch_recipe(&mut self, url: Url) -> Result<StoreResponse, StoreError> {
        let fetcher = Fetcher::from(url);
        let (recipe, ingredients) = fetcher.fetch_recipe().await?;

        self.add_recipe(&recipe, &ingredients).await?;
        Ok(StoreResponse::FetchedRecipe((recipe, ingredients)))
    }

    // We need to deconstruct the `enum` anyway, and so while we do, we handle
    // migrating regardless of which database store has been set via CLI options.
    async fn migrate_json_store_to_sqlite(&mut self) -> Result<StoreResponse, StoreError> {
        match self {
            Self::Json(store) => {
                let mut sqlite_store = SqliteStore::new(DbUri::new()).await?;
                let mut connection = sqlite_store.connection()?;
                let grocery_items = store.items().await?;
                let recipes = store.recipes().await?;
                tokio::task::spawn_blocking(move || {
                    connection.immediate_transaction(|connection| {
                        migrate_sections(connection)?;
                        migrate_recipes(connection, recipes)?;
                        groceries(connection, grocery_items)?;
                        Ok(StoreResponse::JsonToSqlite)
                    })
                })
                .await?
            }
            Self::Sqlite(store) => {
                let mut connection = store.connection()?;
                let grocery_items = store.items().await?;
                connection.immediate_transaction(|connection| {
                    groceries(connection, grocery_items)?;
                    Ok(StoreResponse::JsonToSqlite)
                })
            }
        }
    }
}

#[async_trait::async_trait]
impl Storage for Store {
    async fn add_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_item(item).await,
            Self::Sqlite(store) => store.add_item(item).await,
        }
    }

    async fn add_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_checklist_item(item).await,
            Self::Sqlite(store) => store.add_checklist_item(item).await,
        }
    }

    async fn add_list_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_list_item(item).await,
            Self::Sqlite(store) => store.add_list_item(item).await,
        }
    }

    async fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_list_recipe(recipe).await,
            Self::Sqlite(store) => store.add_list_recipe(recipe).await,
        }
    }

    async fn add_recipe(
        &mut self,
        recipe: &Recipe,
        ingredients: &Ingredients,
    ) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_recipe(recipe, ingredients).await,
            Self::Sqlite(store) => store.add_recipe(recipe, ingredients).await,
        }
    }

    async fn checklist(&mut self) -> Result<Vec<Item>, StoreError> {
        match self {
            Self::Json(store) => store.checklist().await,
            Self::Sqlite(store) => store.checklist().await,
        }
    }

    async fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.delete_checklist_item(item).await,
            Self::Sqlite(store) => store.delete_checklist_item(item).await,
        }
    }

    async fn delete_recipe(&mut self, recipe: &Recipe) -> Result<Recipe, StoreError> {
        match self {
            Self::Json(store) => store.delete_recipe(recipe).await,
            Self::Sqlite(store) => store.delete_recipe(recipe).await,
        }
    }

    async fn items(&mut self) -> Result<Items, StoreError> {
        match self {
            Self::Json(store) => store.items().await,
            Self::Sqlite(store) => store.items().await,
        }
    }

    async fn list(&mut self) -> Result<List, StoreError> {
        match self {
            Self::Json(store) => store.list().await,
            Self::Sqlite(store) => store.list().await,
        }
    }

    async fn refresh_list(&mut self) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.refresh_list().await,
            Self::Sqlite(store) => store.refresh_list().await,
        }
    }

    async fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        match self {
            Self::Json(store) => store.recipes().await,
            Self::Sqlite(store) => store.recipes().await,
        }
    }

    async fn recipe_ingredients(
        &mut self,
        recipe: &Recipe,
    ) -> Result<Option<Ingredients>, StoreError> {
        match self {
            Self::Json(store) => store.recipe_ingredients(recipe).await,
            Self::Sqlite(store) => store.recipe_ingredients(recipe).await,
        }
    }

    async fn sections(&mut self) -> Result<Vec<Section>, StoreError> {
        match self {
            Self::Json(store) => store.sections().await,
            Self::Sqlite(store) => store.sections().await,
        }
    }
}

#[async_trait::async_trait]
pub trait Storage {
    // Create
    async fn add_item(&mut self, item: &Name) -> Result<(), StoreError>;

    async fn add_checklist_item(&mut self, item: &Name) -> Result<(), StoreError>;

    async fn add_list_item(&mut self, item: &Name) -> Result<(), StoreError>;

    async fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError>;

    async fn add_recipe(
        &mut self,
        recipe: &Recipe,
        ingredients: &Ingredients,
    ) -> Result<(), StoreError>;

    // Read
    async fn checklist(&mut self) -> Result<Vec<Item>, StoreError>;

    async fn list(&mut self) -> Result<List, StoreError>;

    async fn items(&mut self) -> Result<Items, StoreError>;

    async fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError>;

    async fn recipe_ingredients(
        &mut self,
        recipe: &Recipe,
    ) -> Result<Option<Ingredients>, StoreError>;

    async fn sections(&mut self) -> Result<Vec<Section>, StoreError>;

    // Update
    async fn refresh_list(&mut self) -> Result<(), StoreError>;

    // Delete
    async fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError>;

    async fn delete_recipe(&mut self, recipe: &Recipe) -> Result<Recipe, StoreError>;
}
