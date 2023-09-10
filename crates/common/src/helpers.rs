use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::errors::ReadError;

pub trait ItemInfo {
    fn name(&self) -> &str;
}

fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, ReadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub trait ReadWrite {
    fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Self, ReadError>
    where
        Self: std::marker::Sized,
        for<'de> Self: serde::Deserialize<'de>,
    {
        let reader = read(path)?;

        Ok(serde_json::from_reader(reader)?)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ReadError>
    where
        Self: serde::Serialize,
    {
        let s = serde_json::to_string(&self)?;

        Ok(fs::write(path, s)?)
    }
}
