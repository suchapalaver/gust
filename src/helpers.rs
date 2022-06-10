use std::{
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

use crate::ReadError;

pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, ReadError> {
    let file: File = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn prompt_for_y() -> Result<bool, ReadError> {
    Ok("y" == get_user_input()?)
}

pub fn get_user_input() -> Result<String, ReadError> {
    stdout().flush()?;

    let mut input = String::new();

    stdin().read_line(&mut input)?;

    let output = input.trim().to_string();

    Ok(output)
}

pub fn write<P: AsRef<Path>>(path: P, object: String) -> Result<(), ReadError> {
    Ok(fs::write(path, &object)?)
}
