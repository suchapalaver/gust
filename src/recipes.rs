use std::{str::FromStr, ops::Deref, fmt};

use crate::{GroceriesItemName, ReadError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct RecipeName(pub String);

impl fmt::Display for RecipeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RecipeName {
    pub fn new(s: &str) -> Result<Self, ReadError> {
        Self::from_str(s)
    }
}

impl FromStr for RecipeName {
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, ReadError> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(pub Vec<GroceriesItemName>);

impl Ingredients {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, elem: GroceriesItemName) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: &str) -> Result<Self, ReadError> {
        Self::from_str(s)
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
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',')
            .map(|item| GroceriesItemName(item.trim().to_lowercase()))
            .collect())
    }
}

impl Deref for Ingredients {
    type Target = Vec<GroceriesItemName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
