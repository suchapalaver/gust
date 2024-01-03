use core::fmt;

use serde::{Deserialize, Serialize};

pub const SECTIONS: [&str; 5] = ["fresh", "pantry", "protein", "dairy", "freezer"];

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Section(String);

impl Section {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Section {
    fn from(value: &str) -> Self {
        Self(value.trim().to_lowercase())
    }
}

impl From<String> for Section {
    fn from(value: String) -> Self {
        Self(value.trim().to_lowercase())
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
