use std::{
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

use crate::ReadError;

// Reads file from path into a read-only buffer-reader
pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, ReadError> {
    let file: File = File::open(path).map_err(|source| ReadError::ReadWriteError { source })?;

    let reader = BufReader::new(file);

    Ok(reader)
}

// Gets user input when it's 'y' or anything else
pub fn prompt_for_y() -> Result<bool, ReadError> {
    Ok("y" == input()?)
}

// Function for getting user input
pub fn input() -> Result<String, ReadError> {
    stdout().flush()?;

    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let output = input.trim().to_string();

    Ok(output)
}

// Writes a String to a path
pub fn write<P: AsRef<Path>>(path: P, object: String) -> Result<(), ReadError> {
    let _ = fs::write(path, &object)?;
    Ok(())
}
