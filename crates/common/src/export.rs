use std::{fs::File, path::Path};

use serde::Serialize;
use thiserror::Error;

use crate::{item::Item, list::List};

pub const ITEMS_YAML_PATH: &str = "items.yaml";
pub const LIST_YAML_PATH: &str = "list.yaml";

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("'serde-yaml' error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
}

pub trait YamlSerializable {
    fn serialize_to_yaml_and_write<P>(&self, path: P) -> Result<(), ExportError>
    where
        P: AsRef<Path>,
        Self: Serialize,
    {
        let file = File::create(path)?;
        serde_yaml::to_writer(file, self)?;

        Ok(())
    }
}

impl YamlSerializable for Vec<Item> {}
impl YamlSerializable for List {}
