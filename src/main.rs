extern crate termion;

use termion::clear;

use std::io::{Read, Write, stdout, stdin};
use std::env;

pub fn main() {
    println!("{}", clear::All);
    let mut data: Vec<u8> = vec!();
    stdin().read_to_end(&mut data).unwrap();
    println!("{}", String::from_utf8(data).unwrap());
}
