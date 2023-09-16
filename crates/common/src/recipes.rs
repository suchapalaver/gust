use std::{fmt, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{item::ItemName, ReadError};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Recipe(String);

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Recipe {
    pub fn new(s: &str) -> Result<Self, ReadError> {
        Self::from_str(s)
    }

    pub fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Recipe {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl FromStr for Recipe {
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().to_lowercase()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(Vec<ItemName>);

impl Ingredients {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, elem: ItemName) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: &str) -> Self {
        Self::from(s)
    }
}

impl FromIterator<ItemName> for Ingredients {
    fn from_iter<I: IntoIterator<Item = ItemName>>(iter: I) -> Self {
        let mut c = Ingredients::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl From<&str> for Ingredients {
    fn from(s: &str) -> Self {
        s.split(',').map(ItemName::from).collect()
    }
}

impl Deref for Ingredients {
    type Target = Vec<ItemName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
