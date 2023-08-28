use std::fs::File;
use std::io::{Read, Result};
use bincode;
// use crate::opening_book::OpeningBook;

pub fn read_binary_opening_book(file_path: &str) {
    let mut file = File::open(file_path);
    let mut buffer = Vec::new();
    file.expect("iiiiii error").read_to_end(&mut buffer);

    let opening_book = bincode::deserialize::<String>(&buffer);
    println!("{}", opening_book.unwrap());
    // Ok(opening_book)
}