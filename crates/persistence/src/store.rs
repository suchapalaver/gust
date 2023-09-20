use common::{
    item::{Item, Name, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
    LoadError, ReadError,
};
use diesel::ConnectionError;
use thiserror::Error;

use std::error::Error;

use crate::{
    json::{migrate::groceries, JsonStore},
    sqlite::{self, establish_connection, SqliteStore},
};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("SQLite database connection error: {0}")]
    ConnectionError(#[from] ConnectionError),

    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),

    #[error("invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("load error: {0}")]
    LoadError(#[from] LoadError),

    #[error("migration error: {0}")]
    MigrationError(#[from] Box<dyn Error + Send + Sync>),

    #[error("read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),

    #[error("ingredients not found for: {0}")]
    RecipeIngredients(String),
}

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
    pub fn new(store: &str) -> Result<Self, StoreError> {
        match store {
            "sqlite" => {
                let mut store = SqliteStore::new(establish_connection()?);
                sqlite::run_migrations(store.connection())?;
                Ok(Store::from(store))
            }
            "json" => Ok(Store::from(JsonStore::default())),

            _ => unreachable!("Store types are currently limited to 'sqlite' and 'json'."),
        }
    }

    // We need to deconstruct the `enum` anyway, and so while we do, we handle
    // migrating regardless of which database store has been set via CLI options.
    pub fn migrate_json_store_to_sqlite(&mut self) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => groceries(
                store,
                SqliteStore::new(establish_connection()?).connection(),
            )?,
            Self::Sqlite(store) => {
                groceries(&mut JsonStore::default(), store.connection())?;
            }
        }
        Ok(())
    }
}

impl Storage for Store {
    fn add_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_item(item),
            Self::Sqlite(store) => store.add_item(item),
        }
    }

    fn add_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_checklist_item(item),
            Self::Sqlite(store) => store.add_checklist_item(item),
        }
    }

    fn add_list_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_list_item(item),
            Self::Sqlite(store) => store.add_list_item(item),
        }
    }

    fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_list_recipe(recipe),
            Self::Sqlite(store) => store.add_list_recipe(recipe),
        }
    }

    fn add_recipe(&mut self, recipe: &Recipe, ingredients: &Ingredients) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_recipe(recipe, ingredients),
            Self::Sqlite(store) => store.add_recipe(recipe, ingredients),
        }
    }

    fn checklist(&mut self) -> Result<Vec<Item>, StoreError> {
        match self {
            Self::Json(store) => store.checklist(),
            Self::Sqlite(store) => store.checklist(),
        }
    }

    fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.delete_checklist_item(item),
            Self::Sqlite(store) => store.delete_checklist_item(item),
        }
    }

    fn delete_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.delete_recipe(recipe),
            Self::Sqlite(store) => store.delete_recipe(recipe),
        }
    }

    fn items(&mut self) -> Result<Items, StoreError> {
        match self {
            Self::Json(store) => store.items(),
            Self::Sqlite(store) => store.items(),
        }
    }

    fn list(&mut self) -> Result<List, StoreError> {
        match self {
            Self::Json(store) => store.list(),
            Self::Sqlite(store) => store.list(),
        }
    }

    fn list_recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        match self {
            Self::Json(store) => store.list_recipes(),
            Self::Sqlite(store) => store.list_recipes(),
        }
    }

    fn refresh_list(&mut self) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.refresh_list(),
            Self::Sqlite(store) => store.refresh_list(),
        }
    }

    fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        match self {
            Self::Json(store) => store.recipes(),
            Self::Sqlite(store) => store.recipes(),
        }
    }

    fn recipe_ingredients(&mut self, recipe: &Recipe) -> Result<Option<Ingredients>, StoreError> {
        match self {
            Self::Json(store) => store.recipe_ingredients(recipe),
            Self::Sqlite(store) => store.recipe_ingredients(recipe),
        }
    }

    fn sections(&mut self) -> Result<Vec<Section>, StoreError> {
        match self {
            Self::Json(store) => store.sections(),
            Self::Sqlite(store) => store.sections(),
        }
    }
}

pub trait Storage {
    // Create
    fn add_item(&mut self, item: &Name) -> Result<(), StoreError>;

    fn add_checklist_item(&mut self, item: &Name) -> Result<(), StoreError>;

    fn add_list_item(&mut self, item: &Name) -> Result<(), StoreError>;

    fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError>;

    fn add_recipe(&mut self, recipe: &Recipe, ingredients: &Ingredients) -> Result<(), StoreError>;

    // Read
    fn checklist(&mut self) -> Result<Vec<Item>, StoreError>;

    fn list(&mut self) -> Result<List, StoreError>;

    fn list_recipes(&mut self) -> Result<Vec<Recipe>, StoreError>;

    fn items(&mut self) -> Result<Items, StoreError>;

    fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError>;

    fn recipe_ingredients(&mut self, recipe: &Recipe) -> Result<Option<Ingredients>, StoreError>;

    fn sections(&mut self) -> Result<Vec<Section>, StoreError>;

    // Update
    fn refresh_list(&mut self) -> Result<(), StoreError>;

    // Delete
    fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError>;

    fn delete_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError>;
}
