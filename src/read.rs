use std::fs::{File};
use std::io::Read;

pub fn get_file_as_bytes(filename: &String) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(filename)
}