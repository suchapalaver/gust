use std::{path::Path, io::{stdin, stdout, BufReader, Write}, fs::{self, File}, error::Error};

use crate::ReadError;

// Reads from a path into a buffer-reader
pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file: File =
        File::open(path).map_err(|err_msg| ReadError::PathError(Box::from(err_msg)))?;

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
