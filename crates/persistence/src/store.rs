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

use std::{error::Error, fmt::Debug, str::FromStr};

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
    SqliteInMem,
}

impl FromStr for StoreType {
    type Err = StoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "sqlite" => Ok(Self::Sqlite),
            "sqlite-inmem" => Ok(Self::SqliteInMem),
            _ => Err(StoreError::ParseStoreType(
                "Store types are currently limited to 'sqlite', 'sqlite-inmem', and 'json'."
                    .to_string(),
            )),
        }
    }
}

#[derive(Clone)]
pub enum Store {
    Json(JsonStore),
    Sqlite(SqliteStore),
}

impl Store {
    pub async fn from_store_type(store_type: StoreType) -> Result<Self, StoreError> {
        use StoreType::*;
        match store_type {
            Json => Ok(Self::Json(JsonStore::default())),
            Sqlite => Ok(Self::Sqlite(SqliteStore::new(DbUri::new()).await?)),
            SqliteInMem => Ok(Self::Sqlite(SqliteStore::new(DbUri::inmem()).await?)),
        }
    }

    pub async fn init(&mut self) -> Result<StoreDispatch, StoreError> {
        let (tx, mut rx) = mpsc::channel::<(
            ApiCommand,
            oneshot::Sender<Result<StoreResponse, StoreError>>,
        )>(10);

        let mut store = self.clone();

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
                                    warn!(?e, "Send reply to API command executor failed");
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
        match self {
            Self::Json(store) => store.execute_transaction(command).await,
            Self::Sqlite(store) => store.execute_transaction(command).await,
        }
    }
}

#[derive(Clone)]
pub struct StoreDispatch {
    tx: mpsc::Sender<(
        ApiCommand,
        oneshot::Sender<Result<StoreResponse, StoreError>>,
    )>,
}

impl StoreDispatch {
    pub fn new(
        tx: mpsc::Sender<(
            ApiCommand,
            oneshot::Sender<Result<StoreResponse, StoreError>>,
        )>,
    ) -> Self {
        Self { tx }
    }

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

#[derive(Debug)]
pub enum StoreResponse {
    AddedChecklistItem(Name),
    AddedItem(Name),
    AddedListItem(Name),
    AddedListRecipe(Recipe),
    AddedRecipe(Recipe),
    Checklist(Vec<Item>),
    DeletedRecipe(Recipe),
    DeletedChecklistItem(Name),
    FetchedRecipe((Recipe, Ingredients)),
    ItemAlreadyAdded(Name),
    Items(Items),
    JsonToSqlite,
    List(List),
    NothingReturned(ApiCommand),
    Recipes(Vec<Recipe>),
    RecipeIngredients(Option<Ingredients>),
    RefreshList,
    Sections(Vec<Section>),
}

pub(crate) trait Storage: Send + Sync + 'static {
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
            Add::ChecklistItem(name) => self.add_checklist_item(&name).await,
            Add::Item { name, .. } => self.add_item(&name).await,
            Add::ListItem(name) => self.add_list_item(&name).await,
            Add::ListRecipe(name) => self.add_list_recipe(&name).await,
            Add::Recipe {
                recipe,
                ingredients,
            } => self.add_recipe(&recipe, &ingredients).await,
        }
    }

    async fn read(&mut self, cmd: Read) -> Result<StoreResponse, StoreError> {
        match cmd {
            Read::All => self.items().await,
            Read::Checklist => self.checklist().await,
            Read::Item(_name) => todo!(),
            Read::List => self.list().await,
            Read::ListRecipes => todo!(),
            Read::Recipe(recipe) => self.recipe_ingredients(&recipe).await,
            Read::Recipes => self.recipes().await,
            Read::Sections => self.sections().await,
        }
    }

    async fn update(&mut self, cmd: Update) -> Result<StoreResponse, StoreError> {
        match cmd {
            Update::Item(_name) => todo!(),
            Update::RefreshList => self.refresh_list().await,
            Update::Recipe(_name) => todo!(),
        }
    }

    async fn delete(&mut self, cmd: Delete) -> Result<StoreResponse, StoreError> {
        match cmd {
            Delete::ChecklistItem(name) => self.delete_checklist_item(&name).await,
            Delete::ClearChecklist => todo!(),
            Delete::ClearList => todo!(),
            Delete::Item(_name) => todo!(),
            Delete::ListItem(_name) => todo!(),
            Delete::Recipe(recipe) => self.delete_recipe(&recipe).await,
        }
    }

    async fn fetch_recipe(&mut self, url: Url) -> Result<StoreResponse, StoreError> {
        let fetcher = Fetcher::from(url);
        let (recipe, ingredients) = fetcher.fetch_recipe().await?;

        self.add_recipe(&recipe, &ingredients).await?;
        Ok(StoreResponse::FetchedRecipe((recipe, ingredients)))
    }

    async fn migrate_json_store_to_sqlite(&mut self) -> Result<StoreResponse, StoreError> {
        let mut sqlite_store = SqliteStore::new(DbUri::new()).await?;
        let mut connection = sqlite_store.connection()?;
        let StoreResponse::Items(grocery_items) = self.items().await? else {
            todo!()
        };
        let StoreResponse::Recipes(recipes) = self.recipes().await? else {
            todo!()
        };
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

    // Create
    async fn add_item(&mut self, item: &Name) -> Result<StoreResponse, StoreError>;

    async fn add_checklist_item(&mut self, item: &Name) -> Result<StoreResponse, StoreError>;

    async fn add_list_item(&mut self, item: &Name) -> Result<StoreResponse, StoreError>;

    async fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<StoreResponse, StoreError>;

    async fn add_recipe(
        &mut self,
        recipe: &Recipe,
        ingredients: &Ingredients,
    ) -> Result<StoreResponse, StoreError>;

    // Read
    async fn checklist(&mut self) -> Result<StoreResponse, StoreError>;

    async fn list(&mut self) -> Result<StoreResponse, StoreError>;

    async fn items(&mut self) -> Result<StoreResponse, StoreError>;

    async fn recipes(&mut self) -> Result<StoreResponse, StoreError>;

    async fn recipe_ingredients(&mut self, recipe: &Recipe) -> Result<StoreResponse, StoreError>;

    async fn sections(&mut self) -> Result<StoreResponse, StoreError>;

    // Update
    async fn refresh_list(&mut self) -> Result<StoreResponse, StoreError>;

    // Delete
    async fn delete_checklist_item(&mut self, item: &Name) -> Result<StoreResponse, StoreError>;

    async fn delete_recipe(&mut self, recipe: &Recipe) -> Result<StoreResponse, StoreError>;
}
