// use std::env;
// use std::fs;
// use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    // let b = Path::new("test.txt").is_file();
    let b = Path::new("test.txt").exists();
    println!("File exists ? {}", b);
}
