use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::store::StoreError;

fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, StoreError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub trait ReadWrite {
    fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Self, StoreError>
    where
        for<'de> Self: serde::Deserialize<'de>,
    {
        let reader = read(path)?;

        Ok(serde_json::from_reader(reader)?)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), StoreError>
    where
        Self: serde::Serialize,
    {
        let s = serde_json::to_string(&self)?;

        Ok(fs::write(path, s)?)
    }
}
