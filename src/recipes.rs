use crate::GroceriesItemName;
use crate::ReadError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Recipe(pub String);

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Recipe {
    pub fn new(s: String) -> Result<Self, ReadError> {
        Recipe::from_str(&s)
    }
}

impl FromStr for Recipe {
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, ReadError> {
        Ok(Recipe(s.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(pub Vec<GroceriesItemName>);

impl Ingredients {
    fn new() -> Ingredients {
        Ingredients(Vec::new())
    }

    fn add(&mut self, elem: GroceriesItemName) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: String) -> Result<Self, ReadError> {
        Ingredients::from_str(&s)
    }
}

impl FromIterator<GroceriesItemName> for Ingredients {
    fn from_iter<I: IntoIterator<Item = GroceriesItemName>>(iter: I) -> Self {
        let mut c = Ingredients::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl FromStr for Ingredients {
    type Err = crate::errors::ReadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',')
            .map(|item| GroceriesItemName(item.trim().to_lowercase()))
            .collect())
    }
}

impl Deref for Ingredients {
    type Target = Vec<crate::GroceriesItemName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
