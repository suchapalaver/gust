use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Invalid JSON file: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("Invalid input")]
    ParseInputError,

    #[error("Error reading/writing file: {0}")]
    ReadWriteError(#[from] std::io::Error),

    #[error("Item not found")]
    ItemNotFound,

    #[error(
        "No groceries library found.\nRun grusterylist groceries to create a groceries library"
    )]
    LibraryNotFound,
}
