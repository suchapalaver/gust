use serde::{Deserialize, Serialize};

use crate::{item::Item, load::Load};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Items(Vec<Item>);

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
        &self.0
    }

    pub fn collection_iter(&self) -> impl Iterator<Item = &Item> {
        self.0.iter()
    }

    pub fn add_item(&mut self, item: Item) {
        if !self.0.iter().any(|i| i.name() == item.name()) {
            self.0.push(item);
        }
    }
}
