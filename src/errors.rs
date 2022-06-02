custom_error::custom_error! {pub ModelError
    DeserializingError{serde_json::Error} = "Invalid JSON file",
    PathError{Box<dyn Error>} = "Invalid file path",
}
