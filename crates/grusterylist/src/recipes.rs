use std::{fmt, ops::Deref, str::FromStr};

use crate::{CliError, ItemName, ReadError};
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().to_lowercase()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(pub Vec<ItemName>);

impl Ingredients {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, elem: ItemName) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: &str) -> Result<Self, CliError> {
        Self::try_from(s)
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

impl TryFrom<&str> for Ingredients {
    type Error = CliError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(s.split(',')
            .map(|item| ItemName(item.trim().to_lowercase()))
            .collect())
    }
}

impl Deref for Ingredients {
    type Target = Vec<ItemName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
