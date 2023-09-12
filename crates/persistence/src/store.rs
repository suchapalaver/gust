use common::{
    item::ItemName,
    recipes::{Ingredients, RecipeName},
};
use thiserror::Error;

use crate::{
    json_db::JsonStore,
    models::{Item, Recipe, Section},
    sqlite_db::SqliteStore,
};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),
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
    fn add_item(&mut self, item: &ItemName) {
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

    fn list(&mut self) -> Vec<Item> {
        todo!()
    }

    fn delete_checklist_item(&mut self, item: &ItemName) {
        todo!()
    }

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
        todo!()
    }

    fn items(&mut self) -> Vec<Item> {
        todo!()
    }

    fn recipe_ingredients(&mut self, recipe: &RecipeName) -> Vec<(RecipeName, Ingredients)> {
        todo!()
    }

    fn sections(&mut self) -> Vec<Section> {
        todo!()
    }

    fn recipes(&mut self) -> Vec<Recipe> {
        todo!()
    }
}

pub trait Storage {
    fn add_item(&mut self, item: &ItemName);

    fn add_checklist_item(&mut self, item: &ItemName);

    fn add_list_item(&mut self, item: &ItemName);

    fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients);

    fn checklist(&mut self) -> Vec<Item>;

    fn list(&mut self) -> Vec<Item>;

    fn delete_checklist_item(&mut self, item: &ItemName);

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError>;

    fn items(&mut self) -> Vec<Item>;

    fn recipe_ingredients(&mut self, recipe: &RecipeName) -> Vec<(RecipeName, Ingredients)>;

    fn sections(&mut self) -> Vec<Section>;

    fn recipes(&mut self) -> Vec<Recipe>;
}
