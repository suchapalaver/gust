use common::{
    item::{ Item, Name, Section },
    items::Items,
    list::List,
    recipes::{ Ingredients, Recipe },
    LoadError,
    ReadError,
};
use diesel::{ r2d2::{ ConnectionManager, Pool }, SqliteConnection };
use thiserror::Error;

use std::{ env, error::Error, ops::Deref, str::FromStr };

use crate::{
    json::{ migrate::{ groceries, migrate_recipes, migrate_sections }, JsonStore },
    sqlite::{ self, SqliteStore },
};

#[rustfmt::skip]
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
}

pub struct DbUri(String);

impl From<&str> for DbUri {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Deref for DbUri {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn db_uri() -> DbUri {
    dotenvy::dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set").as_str().into()
}

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

#[async_trait::async_trait]
pub trait Connection {
    async fn try_connect(&self) -> Result<ConnectionPool, StoreError>;
}

pub(crate) struct DatabaseConnector {
    db_uri: DbUri,
}

impl DatabaseConnector {
    pub(crate) fn new(db_uri: DbUri) -> Self {
        Self { db_uri }
    }
}

#[async_trait::async_trait]
impl Connection for DatabaseConnector {
    async fn try_connect(&self) -> Result<ConnectionPool, StoreError> {
        use diesel::Connection;
        SqliteConnection::establish(&self.db_uri)?;
        Ok(Pool::builder().build(ConnectionManager::<SqliteConnection>::new(self.db_uri.deref()))?)
    }
}

#[derive(Debug)]
pub enum StoreType {
    Json,
    Sqlite,
}

impl FromStr for StoreType {
    type Err = StoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "sqlite" => Ok(Self::Sqlite),
            _ =>
                Err(
                    StoreError::ParseStoreType(
                        "Store types are currently limited to 'sqlite' and 'json'.".to_string()
                    )
                ),
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

impl Store {
    pub async fn new(store: StoreType) -> Result<Self, StoreError> {
        match store {
            StoreType::Sqlite => {
                let db_uri = db_uri();
                let connection_pool = DatabaseConnector::new(db_uri).try_connect().await?;
                let mut store = SqliteStore::new(connection_pool);
                let mut connection = store.connection()?;
                connection.immediate_transaction(sqlite::run_migrations)?;
                Ok(Store::from(store))
            }
            StoreType::Json => Ok(Store::from(JsonStore::default())),
        }
    }

    pub async fn new_inmem(store: StoreType) -> Result<Self, StoreError> {
        match store {
            StoreType::Sqlite => {
                let db_uri = DbUri::from(":memory:");
                let connection_pool = DatabaseConnector::new(db_uri).try_connect().await?;
                let mut store = SqliteStore::new(connection_pool);
                let mut connection = store.connection()?;
                connection.immediate_transaction(sqlite::run_migrations)?;
                Ok(Store::from(store))
            }
            StoreType::Json => Ok(Store::from(JsonStore::default())),
        }
    }

    // We need to deconstruct the `enum` anyway, and so while we do, we handle
    // migrating regardless of which database store has been set via CLI options.
    pub async fn migrate_json_store_to_sqlite(&mut self) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => {
                let db_uri = db_uri();
                let connection_pool = DatabaseConnector::new(db_uri).try_connect().await?;
                let mut sqlite_store = SqliteStore::new(connection_pool);
                let mut connection = sqlite_store.connection()?;
                let grocery_items = store.items().await?;
                let recipes = store.recipes().await?;
                tokio::task::spawn_blocking(move || {
                    connection.immediate_transaction(|connection| {
                        migrate_sections(connection)?;
                        migrate_recipes(connection, recipes)?;
                        groceries(connection, grocery_items)?;
                        Ok(())
                    })
                }).await?
            }
            Self::Sqlite(store) => {
                let mut connection = store.connection()?;
                let grocery_items = store.items().await?;
                connection.immediate_transaction(|connection| {
                    groceries(connection, grocery_items)?;
                    Ok(())
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
        ingredients: &Ingredients
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
        recipe: &Recipe
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
        ingredients: &Ingredients
    ) -> Result<(), StoreError>;

    // Read
    async fn checklist(&mut self) -> Result<Vec<Item>, StoreError>;

    async fn list(&mut self) -> Result<List, StoreError>;

    async fn items(&mut self) -> Result<Items, StoreError>;

    async fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError>;

    async fn recipe_ingredients(
        &mut self,
        recipe: &Recipe
    ) -> Result<Option<Ingredients>, StoreError>;

    async fn sections(&mut self) -> Result<Vec<Section>, StoreError>;

    // Update
    async fn refresh_list(&mut self) -> Result<(), StoreError>;

    // Delete
    async fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError>;

    async fn delete_recipe(&mut self, recipe: &Recipe) -> Result<Recipe, StoreError>;
}
