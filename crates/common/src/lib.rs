pub mod commands;
pub mod groceries;
pub mod groceriesitem;
pub mod helpers;
pub mod input;
pub mod recipes;
pub mod run_shopping_list;
pub mod scraper;
pub mod sections;
pub mod shoppinglist;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("Error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),

    #[error("Item not found")]
    ItemNotFound,

    #[error("No groceries library found")]
    LibraryNotFound,
}
