use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{BufRead, BufReader, Error};

use super::walking_vec::WalkingVec;

pub fn read_file(path: &str) -> Option<File> {
    OpenOptions::new().read(true).open(path).ok()
}

pub fn read_first_line_in_file(path: &str) -> Result<String, Error> {
    let mut first_line = String::new();
    let file: Result<File, Error> = OpenOptions::new().read(true).open(path);

    BufReader::new(file?).read_line(&mut first_line)?;
    // Delete the newline character at the end of the file because we don't need it
    first_line.pop();
    Ok(first_line)
}
pub fn read_file_to_vec(path: &str) -> Result<WalkingVec, Error> {
    let mut file = File::open(path)?;
    let metadata = std::fs::metadata(path)?;
    // Create a vector that has the same size as the tz file
    // and fill it with zeros
    let mut bytes = vec![0; metadata.len() as usize];
    file.read(&mut bytes)?;

    Ok(WalkingVec {
        buffer: bytes,
        position: 0,
    })
}
