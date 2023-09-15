pub mod commands;
pub mod input;
pub mod item;
pub mod items;
pub mod list;
pub mod recipes;
pub mod run_list;
pub mod scraper;
pub mod sections;

use std::{
    io::{self},
    path::Path,
};

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Invalid JSON: {0}")]
    Json(Value),

    #[error("Item not found")]
    ItemNotFound,

    #[error("No groceries library found")]
    LibraryNotFound,
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Load error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub trait Load {
    type T: for<'a> Deserialize<'a>;

    fn from_json<P: AsRef<Path>>(path: P) -> Result<Self::T, LoadError> {
        let reader = Self::reader(path)?;
        Ok(Self::from_reader(&reader)?)
    }

    fn reader<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
        let file = std::fs::read_to_string(path)?;
        Ok(file)
    }

    fn from_reader(reader: &str) -> Result<Self::T, serde_json::Error> {
        serde_json::from_str(reader)
    }
}
