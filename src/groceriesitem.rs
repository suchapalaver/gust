use std::{error::Error, fmt, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::Recipe;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItem {
    pub name: GroceriesItemName,       // e.g. "apples"
    pub section: GroceriesItemSection, // e.g. "fresh"
    pub is_recipe_ingredient: bool,    // i.e. true
    pub recipes: Vec<Recipe>,          // list of recipes: "apple pie", "cheese plate", ...
}

impl GroceriesItem {
    pub fn new(name_and_section: Vec<String>) -> Result<Self, Box<dyn Error>> {
        // this fn def is duplicate
        Self::new_initialized(name_and_section)
    }

    pub fn new_initialized(name_and_section: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let name = name_and_section.get(0).expect("no grocery name found!");
        let section = name_and_section.get(1).expect("no grocery section found");
        Ok(GroceriesItem {
            name: GroceriesItemName(name.clone()),
            section: GroceriesItemSection(section.clone()),
            is_recipe_ingredient: false,
            recipes: vec![],
        })
    }
}
/*
impl Default for GroceriesItem {
    fn default() -> Self {
        Self::new()
    }
}
*/

impl fmt::Display for GroceriesItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Deref for GroceriesItem {
    type Target = Vec<Recipe>;

    fn deref(&self) -> &Self::Target {
        &self.recipes
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItemName(pub String);

impl std::fmt::Display for GroceriesItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItemSection(pub String);

impl fmt::Display for GroceriesItemSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
