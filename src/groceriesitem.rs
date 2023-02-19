use crate::RecipeName;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: ItemName,           // e.g. "apples"
    pub section: Option<Section>, // e.g. "fresh"
    pub recipes: Option<Vec<RecipeName>>, // list of recipes: "apple pie", "cheese plate", ...
                                  // pub on_list: bool,
                                  // pub on_checklist: bool,
}

impl Item {
    pub fn new(name: &str, section: &str) -> Self {
        Self {
            name: ItemName(name.to_string()),
            ..Default::default()
        }
    }

    pub(crate) fn matches(&self, s: &str) -> bool {
        s.split(' ').all(|word| !self.name.0.contains(word))
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ItemName(pub String);

impl std::fmt::Display for ItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Section(pub String);

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
