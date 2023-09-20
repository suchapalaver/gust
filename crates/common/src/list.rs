use std::str::FromStr;

use crate::{
    input::user_wants_to_add_item_to_list,
    item::{Item, Name, SECTIONS},
    items::Items,
    recipes::Recipe,
    Load, ReadError,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub checklist: Vec<Item>,
    pub recipes: Vec<Recipe>,
    pub items: Vec<Item>,
}

impl Load for List {
    type T = List;
}

impl FromIterator<Item> for List {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = List::new();

        for i in iter {
            c.add_item(i);
        }
        c
    }
}

impl List {
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
                println!("\t{recipe}");
            });
        }
        if !self.items.is_empty() {
            println!("groceries:");

            self.items.iter().for_each(|item| {
                println!("\t{}", item.name.as_str().to_lowercase());
            });
        }
    }

    pub fn add_groceries(&mut self, groceries: &Items) -> Result<(), ReadError> {
        // move everything off list to temp list
        let list_items: Vec<Item> = self.items.drain(..).collect();
        let sections = SECTIONS;
        let groceries_by_section: Vec<Vec<Item>> = {
            sections
                .into_iter()
                .map(|section| {
                    let mut a: Vec<Item> = list_items
                        .iter()
                        .filter(|item| item.section.is_some())
                        .filter(|item| {
                            if let Some(item_sec) = &item.section {
                                item_sec.as_str() == section
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect();

                    let b: Vec<Item> = groceries
                        .collection
                        .iter()
                        .filter(|item| {
                            if let Some(item_sec) = &item.section {
                                item_sec.as_str() == section && !a.contains(item)
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect();
                    a.extend(b);
                    a
                })
                .collect()
        };
        for section in groceries_by_section {
            if !section.is_empty() {
                for item in &section {
                    if !self.items.contains(item) {
                        if let Some(recipes) = &item.recipes {
                            if recipes.iter().any(|recipe| self.recipes.contains(recipe)) {
                                self.add_item(item.clone());
                            }
                        }
                    }
                }
                for item in section {
                    if !self.items.contains(&item) {
                        let res = user_wants_to_add_item_to_list(&item);

                        match res {
                            Some(true) => {
                                if !self.items.contains(&item) {
                                    self.add_item(item.clone());
                                }
                            }
                            Some(false) => continue,
                            None => break,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn delete_groceries_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .items
            .iter()
            .position(|x| x.name == Name::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.items.remove(i);
        }
        Ok(())
    }

    pub fn add_checklist_item(&mut self, item: Item) {
        self.checklist.push(item);
    }

    pub fn delete_checklist_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .checklist
            .iter()
            .position(|x| x.name == Name::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.checklist.remove(i);
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        let recipe = Recipe::from_str(name)?;
        if let Ok(index) = self
            .recipes
            .iter()
            .position(|x| x == &recipe)
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(index);
        }
        Ok(())
    }
}
