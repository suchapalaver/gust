custom_error::custom_error! {pub ReadError
    DeserializingError{ source: serde_json::Error } = "Invalid JSON file",
    ParseInputError = "Invalid input",
    ReadWriteError{ source: std::io::Error } = "Error reading/writing file",
    ItemNotFound = "Item not found",
    LibraryNotFound = "No groceries library found.\nRun grusterylist groceries to create a groceries library",
}
