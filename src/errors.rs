custom_error::custom_error! {pub ReadError
    DeserializingError{ source: serde_json::Error } = "Invalid JSON file",
    ParseInputError = "Invalid input",
    ReadWriteError{ source: std::io::Error } = "Error reading/writing file",
}

// custom_error::custom_error! {pub LookupError
//     ItemNotFound = "Item not found",
//     }
