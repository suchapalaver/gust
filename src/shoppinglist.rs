use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{GroceriesItem, ReadError, Recipe};

// used to serialize and deserialize the
// most recently saved list or to create a
// new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self::new()
    }
}

impl ShoppingList {
    pub fn new() -> Self {
        Self::new_initialized()
    }

    fn new_initialized() -> Self {
        ShoppingList {
            checklist: vec![],
            recipes: vec![],
            groceries: vec![],
        }
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, ReadError> {
        let reader = crate::helpers::read(path)?;

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn print(&self) {
        if !self.checklist.is_empty() {
            println!("Check if we need:");

            self.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        if !self.recipes.is_empty() {
            println!("recipes:");

            self.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !self.groceries.is_empty() {
            println!("groceries:");

            self.groceries.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
    }

    pub fn add_groceries_item(&mut self, item: GroceriesItem) {
        self.groceries.push(item)
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe)
    }

    pub fn save(&self) -> Result<(), ReadError> {
        let json = serde_json::to_string(&self)?;
        crate::helpers::write("list.json", json)
    }
}
