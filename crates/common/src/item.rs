use serde::{Deserialize, Serialize};
use std::fmt;

use crate::recipes::Recipe;

/// An item used in recipes or bought separately
///
/// # Arguments
/// * `name` - name of the item
/// * `section` - section in which item is found ("fresh", "frozen", etc.)
/// * `recipes` - list of recipes of which the item is an ingredient
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    pub name: Name,
    pub section: Option<Section>,
    pub recipes: Option<Vec<Recipe>>,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name(name.into()),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
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
pub struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name(value.trim().to_lowercase())
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub const SECTIONS: [&str; 5] = ["fresh", "pantry", "protein", "dairy", "freezer"];

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
    pub fn new(sec: impl Into<String>) -> Self {
        Self(sec.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
