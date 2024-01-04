use std::{
    io::{self},
    path::Path,
};

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("load error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("'serde-json' error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub trait Load {
    type T: for<'a> Deserialize<'a>;

    fn from_json<P: AsRef<Path>>(path: P) -> Result<Self::T, LoadError>
    where
        Self: for<'a> Deserialize<'a>,
    {
        let reader = Self::reader(path)?;
        Ok(Self::from_reader(&reader)?)
    }

    fn reader<P: AsRef<Path>>(path: P) -> Result<String, io::Error>
    where
        Self: for<'a> Deserialize<'a>,
    {
        let file = std::fs::read_to_string(path)?;
        Ok(file)
    }

    fn from_reader(reader: &str) -> Result<Self::T, serde_json::Error>
    where
        Self: for<'a> Deserialize<'a>,
    {
        serde_json::from_str(reader)
    }
}
