use std::{
    fs::{self},
    path::PathBuf,
};

use common::{items::Items, list::List, Load};

use crate::store::StoreError;

pub const ITEMS_JSON_PATH: &str = "items.json";
pub const LIST_JSON_PATH: &str = "list.json";

#[derive(Clone)]
pub struct ImportStore {
    items: PathBuf,
    list: PathBuf,
}

impl Default for ImportStore {
    fn default() -> Self {
        Self {
            items: PathBuf::from(ITEMS_JSON_PATH),
            list: PathBuf::from(LIST_JSON_PATH),
        }
    }
}

impl ImportStore {
    pub fn items(&self) -> Result<Items, StoreError> {
        Ok(Items::from_json(&self.items)?)
    }

    pub fn list(&self) -> Result<List, StoreError> {
        Ok(List::from_json(&self.list)?)
    }

    pub fn export_items(&self, object: impl serde::Serialize) -> Result<(), StoreError> {
        let s = serde_json::to_string(&object)?;
        Ok(fs::write(&self.items, s)?)
    }

    pub fn export_list(&self, object: impl serde::Serialize) -> Result<(), StoreError> {
        let s = serde_json::to_string(&object)?;
        Ok(fs::write(&self.list, s)?)
    }
}
