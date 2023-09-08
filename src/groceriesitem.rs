use crate::RecipeName;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItem {
    pub name: GroceriesItemName,                // e.g. "apples"
    pub section: Option<GroceriesItemSection>,  // e.g. "fresh"
    pub recipes: Option<Vec<RecipeName>>,       // list of recipes: "apple pie", "cheese plate", ...
                                                // pub on_list: bool,
                                                // pub on_checklist: bool,
}

impl GroceriesItem {
    pub fn new(name: &str, section: &str) -> Self {
        Self {
            name: GroceriesItemName(name.to_string()),
            ..Default::default()
        }
    }

    pub fn matches(&self, s: &str) -> bool {
        s.split(' ').all(|word| !self.name.0.contains(word))
    }
}

impl fmt::Display for GroceriesItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GroceriesItemName(pub String);

impl std::fmt::Display for GroceriesItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GroceriesItemSection(pub String);

impl fmt::Display for GroceriesItemSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
