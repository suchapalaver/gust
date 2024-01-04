use serde::{Deserialize, Serialize};

use crate::{item::Item, load::Load, recipes::Recipe, section::Section};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Items {
    sections: Vec<Section>,
    collection: Vec<Item>,
    recipes: Vec<Recipe>,
}

impl Load for Items {
    type T = Items;
}

impl FromIterator<Item> for Items {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = Items::new();

        for i in iter {
            c.add_item(i);
        }
        c
    }
}

impl Items {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn collection(&self) -> &[Item] {
        &self.collection
    }

    pub fn collection_iter(&self) -> impl Iterator<Item = &Item> {
        self.collection.iter()
    }

    pub fn add_item(&mut self, item: Item) {
        if !self.collection.iter().any(|i| i.name() == item.name()) {
            self.collection.push(item);
        }
    }
}
