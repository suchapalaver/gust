use std::str::FromStr;

use crate::{
    item::{Item, ItemName},
    recipes::RecipeName,
    Load, ReadError,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<Item>,
    pub recipes: Vec<RecipeName>,
    pub items: Vec<Item>,
}

impl Load for ShoppingList {
    type T = ShoppingList;
}

impl FromIterator<Item> for ShoppingList {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = ShoppingList::new();

        for i in iter {
            c.add_item(i);
        }
        c
    }
}

impl ShoppingList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print(&self) {
        if !self.checklist.is_empty() {
            println!("Check if we need:");

            self.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.as_str().to_lowercase());
            });
        }
        if !self.recipes.is_empty() {
            println!("recipes:");

            self.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !self.items.is_empty() {
            println!("groceries:");

            self.items.iter().for_each(|item| {
                println!("\t{}", item.name.as_str().to_lowercase());
            });
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn delete_groceries_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .items
            .iter()
            .position(|x| x.name == ItemName::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.items.remove(i);
        }
        Ok(())
    }

    pub fn add_checklist_item(&mut self, item: Item) {
        self.checklist.push(item)
    }

    pub fn delete_checklist_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .checklist
            .iter()
            .position(|x| x.name == ItemName::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.checklist.remove(i);
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: RecipeName) {
        self.recipes.push(recipe)
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|x| x == &RecipeName::from_str(name).unwrap())
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        Ok(())
    }
}
