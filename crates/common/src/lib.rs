pub mod commands;
pub mod input;
pub mod item;
pub mod items;
pub mod list;
pub mod recipes;
pub mod run_list;
pub mod scraper;
pub mod sections;

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
