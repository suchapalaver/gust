use serde::{Deserialize, Serialize};
use std::fmt;

use crate::recipes::RecipeName;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: ItemName,           // e.g. "apples"
    pub section: Option<Section>, // e.g. "fresh"
    pub recipes: Option<Vec<RecipeName>>, // list of recipes: "apple pie", "cheese plate", ...
                                  // pub on_list: bool,
                                  // pub on_checklist: bool,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: ItemName(name.into()),
            ..Default::default()
        }
    }

    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(Section(section.into()));
        self
    }

    pub(crate) fn matches(&self, s: impl Into<String>) -> bool {
        s.into().split(' ').all(|word| !self.name.0.contains(word))
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ItemName(String);

impl std::fmt::Display for ItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ItemName {
    fn from(value: &str) -> Self {
        ItemName(value.trim().to_lowercase())
    }
}

impl ItemName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Section(String);

impl From<&str> for Section {
    fn from(value: &str) -> Self {
        Self(value.trim().to_lowercase())
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Section {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}