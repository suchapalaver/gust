use serde::de::Error;
use std::error::Error as StdError;

custom_error::custom_error! {pub ReadError
    DeserializingError{ source: serde_json::Error } = "Invalid JSON file",
    PathError{ source: Box<dyn StdError> } = "Invalid file path",
}
