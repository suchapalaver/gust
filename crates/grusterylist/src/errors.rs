use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrusterylistError {
    #[error("Cli error: {0}")]
    CliError(#[from] CliError),

    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid input: {0}")]
    ParseInputError(String),

    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
}

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

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("DB query failed: {0}")]
    DBQuery(#[from] diesel::result::Error),
}
