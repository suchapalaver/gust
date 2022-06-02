use std::{
    error::Error,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

use crate::ReadError;

// Reads file from path into a read-only buffer-reader
pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
    let file: File =
        File::open(path).map_err(|err_msg| ReadError::PathError{ source: Box::from(err_msg) })?;

    let reader = BufReader::new(file);

    Ok(reader)
}

// Gets user input when it's 'y' or anything else
pub fn prompt_for_y() -> Result<bool, Box<dyn Error>> {
    Ok("y" == input()?)
}

// Function for getting user input
pub fn input() -> Result<String, Box<dyn Error>> {
    stdout().flush()?;

    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let output = input.trim().to_string();

    Ok(output)
}

// Writes a String to a path
pub fn write<P: AsRef<Path>>(path: P, object: String) -> Result<(), Box<dyn Error>> {
    let _ = fs::write(path, &object)?;
    Ok(())
}
