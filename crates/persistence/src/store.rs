use common::{
    groceriesitem::ItemName,
    recipes::{Ingredients, RecipeName},
};
use thiserror::Error;

use crate::models::{Item, Recipe, Section};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),
}

pub trait Store {
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
