use std::env;
use std::fs;
use std::io::prelude::*;

pub fn run() {
    let mut file = String::from("test.md");

    if env::args().len() > 1 {
        file = env::args().nth(1).unwrap();
    }

    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon\n");
    speech.push_str("and do the other thing\n");

    let _ = fs::write(file, speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("test.txt")
        .unwrap();

    let _ = file.write(b"\nLast statement");
}
