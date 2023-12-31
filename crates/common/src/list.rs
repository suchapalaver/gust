use crate::{
    input::user_wants_to_add_item_to_list,
    item::{Item, SECTIONS},
    items::Items,
    recipes::Recipe,
    Load,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct List {
    checklist: Vec<Item>,
    recipes: Vec<Recipe>,
    items: Vec<Item>,
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

    pub fn with_checklist(mut self, checklist: Vec<Item>) -> Self {
        self.checklist.extend(checklist);
        self
    }

    pub fn with_recipes(mut self, recipes: Vec<Recipe>) -> Self {
        self.recipes.extend(recipes);
        self
    }

    pub fn with_items(mut self, items: Vec<Item>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn checklist(&self) -> &Vec<Item> {
        &self.checklist
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn print(&self) {
        if !self.checklist.is_empty() {
            println!("Check if we need:");

            self.checklist.iter().for_each(|item| {
                println!("\t{}", item.name());
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
                println!("\t{}", item.name());
            });
        }
    }

    pub fn add_groceries(&mut self, groceries: &Items) {
        // move everything off list to temp list
        let list_items: Vec<Item> = self.items.drain(..).collect();
        let sections = SECTIONS;
        let groceries_by_section: Vec<Vec<Item>> = {
            sections
                .into_iter()
                .map(|section| {
                    let mut a: Vec<Item> = list_items
                        .iter()
                        .filter(|item| item.section().is_some())
                        .filter(|item| {
                            item.section()
                                .map_or(false, |item_sec| item_sec.as_str() == section)
                        })
                        .cloned()
                        .collect();

                    let b: Vec<Item> = groceries
                        .collection()
                        .filter(|item| {
                            item.section().map_or(false, |item_sec| {
                                item_sec.as_str() == section && !a.contains(item)
                            })
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
                        if let Some(recipes) = &item.recipes() {
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
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn delete_groceries_item(&mut self, name: &str) {
        self.items = self
            .items
            .drain(..)
            .filter(|item| item.name().as_str() != name)
            .collect();
    }

    pub fn add_checklist_item(&mut self, item: Item) {
        self.checklist.push(item);
    }

    pub fn delete_checklist_item(&mut self, name: &str) {
        self.checklist = self
            .checklist
            .drain(..)
            .filter(|item| item.name().as_str() != name)
            .collect();
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn delete_recipe(&mut self, name: &str) {
        self.recipes = self
            .recipes
            .drain(..)
            .filter(|recipe| recipe.as_str() != name)
            .collect();
    }
}
