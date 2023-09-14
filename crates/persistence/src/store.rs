use common::{
    item::{Item, ItemName, Section},
    items::Items,
    list::ShoppingList,
    recipes::{Ingredients, RecipeName},
    LoadError,
};
use thiserror::Error;

use crate::{json::JsonStore, sqlite::SqliteStore};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),

    #[error("Invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("Error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),

    #[error("Load error: {0}")]
    LoadError(#[from] LoadError),
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

impl Storage for Store {
    fn add_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_item(item),
            Self::Sqlite(store) => store.add_item(item),
        }
    }

    fn add_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_checklist_item(item),
            Self::Sqlite(store) => store.add_checklist_item(item),
        }
    }

    fn add_list_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.add_list_item(item),
            Self::Sqlite(store) => store.add_list_item(item),
        }
    }

    fn add_recipe(
        &mut self,
        recipe: &RecipeName,
        ingredients: &Ingredients,
    ) -> Result<(), StoreError> {
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

    fn delete_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        match self {
            Self::Json(store) => store.delete_checklist_item(item),
            Self::Sqlite(store) => store.delete_checklist_item(item),
        }
    }

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
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

    fn list(&mut self) -> Result<ShoppingList, StoreError> {
        match self {
            Self::Json(store) => store.list(),
            Self::Sqlite(store) => store.list(),
        }
    }

    fn recipes(&mut self) -> Result<Vec<RecipeName>, StoreError> {
        match self {
            Self::Json(store) => store.recipes(),
            Self::Sqlite(store) => store.recipes(),
        }
    }

    fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Result<Option<Ingredients>, StoreError> {
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
    fn add_item(&mut self, item: &ItemName) -> Result<(), StoreError>;

    fn add_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError>;

    fn add_list_item(&mut self, item: &ItemName) -> Result<(), StoreError>;

    fn add_recipe(
        &mut self,
        recipe: &RecipeName,
        ingredients: &Ingredients,
    ) -> Result<(), StoreError>;

    // Read
    fn checklist(&mut self) -> Result<Vec<Item>, StoreError>;

    fn list(&mut self) -> Result<ShoppingList, StoreError>;

    fn items(&mut self) -> Result<Items, StoreError>;

    fn recipes(&mut self) -> Result<Vec<RecipeName>, StoreError>;

    fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Result<Option<Ingredients>, StoreError>;

    fn sections(&mut self) -> Result<Vec<Section>, StoreError>;

    // Update
    // ...

    // Delete
    fn delete_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError>;

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError>;
}
