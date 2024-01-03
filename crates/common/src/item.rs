use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{recipes::Recipe, section::Section};

/// An item used in recipes or bought separately
///
/// # Arguments
///
/// * `name` - name of the item
/// * `section` - section in which item is found ("fresh", "frozen", etc.)
/// * `recipes` - list of recipes of which the item is an ingredient
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    name: Name,
    section: Option<Section>,
    recipes: Option<Vec<Recipe>>,
}

impl Item {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Name(name.into().to_lowercase()),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn section(&self) -> Option<&Section> {
        self.section.as_ref()
    }

    pub fn recipes(&self) -> Option<&Vec<Recipe>> {
        self.recipes.as_ref()
    }

    pub fn delete_recipe(&mut self, name: &str) {
        if let Some(vec) = self.recipes.as_mut() {
            vec.retain(|x| x.as_str() != name)
        }
    }

    pub fn with_section(mut self, section: &str) -> Self {
        self.section = Some(section.into());
        self
    }

    pub fn with_recipes(mut self, recipes: &[Recipe]) -> Self {
        self.recipes = Some(recipes.to_vec());
        self
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&Name> for Item {
    fn from(name: &Name) -> Self {
        Self::new(name.as_str())
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
