use std::{error::Error, fmt};

// Customized handling of
// file reading errors
#[derive(Debug)]
pub enum ReadError {
    DeserializingError(serde_json::Error),
    PathError(Box<dyn Error>),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::DeserializingError(e) => write!(
                f,
                "Error deserializing from JSON file:\n\
                 '{}'!\n\
		 Something's wrong with the JSON file?\n\
		 See the example json files in the \
		 grusterylist repository to see \
		 how things should look.\n",
                e
            ),
            ReadError::PathError(e) => write!(
                f,
                "Error: '{}'!\n\
		 Make sure file with that path \
		 can be accessed by the \
		 present working directory",
                e
            ),
        }
    }
}

// This is to make compatibility with
// the chain of Box<dyn Error> messaging
impl Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::DeserializingError(_) => "Error deserializing from JSON file!",
            ReadError::PathError(_) => "File does not exist!",
        }
    }
}
