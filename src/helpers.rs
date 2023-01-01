use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::ReadError;

pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, ReadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn write<P: AsRef<Path>>(path: P, object: &str) -> Result<(), ReadError> {
    Ok(fs::write(path, object)?)
}
