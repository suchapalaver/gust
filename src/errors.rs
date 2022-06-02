use serde::de::Error;

custom_error::custom_error! {pub ReadError
    DeserializingError{ source: serde_json::Error } = "Invalid JSON file",
    PathError{ source: Box<dyn Error> } = "Invalid file path",
}
