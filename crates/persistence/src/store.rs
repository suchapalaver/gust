use common::{
    item::ItemName,
    items::Groceries,
    list::ShoppingList,
    recipes::{Ingredients, RecipeName},
};
use thiserror::Error;

use crate::{
    json_db::JsonStore,
    models::{Item, Section},
    sqlite_db::SqliteStore,
};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),

    #[error("Invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("Error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),
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
        todo!()
    }

    fn add_checklist_item(&mut self, item: &ItemName) {
        todo!()
    }

    fn add_list_item(&mut self, item: &ItemName) {
        todo!()
    }

    fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients) {
        todo!()
    }

    fn checklist(&mut self) -> Vec<Item> {
        todo!()
    }

    fn delete_checklist_item(&mut self, item: &ItemName) {
        todo!()
    }

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
        todo!()
    }

    fn items(&mut self) -> Result<Groceries, StoreError> {
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

    fn sections(&mut self) -> Vec<Section> {
        match self {
            Self::Json(store) => store.sections(),
            Self::Sqlite(store) => store.sections(),
        }
    }
}

pub trait Storage {
    fn add_item(&mut self, item: &ItemName) -> Result<(), StoreError>;

    fn add_checklist_item(&mut self, item: &ItemName);

    fn add_list_item(&mut self, item: &ItemName);

    fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients);

    fn checklist(&mut self) -> Vec<Item>;

    fn list(&mut self) -> Result<ShoppingList, StoreError>;

    fn delete_checklist_item(&mut self, item: &ItemName);

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError>;

    fn items(&mut self) -> Result<Groceries, StoreError>;

    fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Result<Option<Ingredients>, StoreError>;

    fn sections(&mut self) -> Vec<Section>;

    fn recipes(&mut self) -> Result<Vec<RecipeName>, StoreError>;
}
