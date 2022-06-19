use crate::Recipe;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItem {
    pub name: GroceriesItemName,       // e.g. "apples"
    pub section: GroceriesItemSection, // e.g. "fresh"
    pub is_recipe_ingredient: bool,    // i.e. true
    pub recipes: Vec<Recipe>,          // list of recipes: "apple pie", "cheese plate", ...
                                       //pub on_list: bool,
                                       //pub on_checklist: bool,
}

impl GroceriesItem {
    pub fn new(name: &str, section: &str) -> Self {
        Self::new_initialized(
            GroceriesItemName(name.to_string()),
            GroceriesItemSection(section.to_string()),
        )
    }

    pub fn new_initialized(name: GroceriesItemName, section: GroceriesItemSection) -> Self {
        //let name = name_and_section.get(0).expect("no grocery name found!");
        //let section = name_and_section.get(1).expect("no grocery section found");
        GroceriesItem {
            name,
            section,
            is_recipe_ingredient: false,
            recipes: vec![],
            //on_list: false,
            //on_checklist: false,
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
