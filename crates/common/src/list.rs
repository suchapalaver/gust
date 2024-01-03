use crate::{item::Item, recipes::Recipe, Load};
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

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}
