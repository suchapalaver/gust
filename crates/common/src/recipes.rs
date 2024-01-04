use std::{fmt, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::item::Name;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Recipe(String);

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Recipe {
    pub fn new(s: &str) -> Self {
        s.into()
    }

    pub fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_input_string(s: &str) -> Self {
        Self::from(s)
    }
}

impl From<&str> for Recipe {
    fn from(s: &str) -> Self {
        Self(s.trim().to_lowercase())
    }
}

impl From<String> for Recipe {
    fn from(s: String) -> Self {
        Self(s.trim().to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(Vec<Name>);

impl Ingredients {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, elem: Name) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: &str) -> Self {
        Self::from(s)
    }
}

impl FromIterator<Name> for Ingredients {
    fn from_iter<I: IntoIterator<Item = Name>>(iter: I) -> Self {
        let mut c = Ingredients::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl From<&str> for Ingredients {
    fn from(s: &str) -> Self {
        s.split(',').map(Name::from).collect()
    }
}

impl Deref for Ingredients {
    type Target = Vec<Name>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
