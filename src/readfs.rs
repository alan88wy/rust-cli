use std::env;
use std::fs;

pub fn run() {
    let mut file = String::from("README.md");

    if env::args().len() > 1 {
        file = env::args().nth(1).unwrap();
    }

    let contents = fs::read_to_string(&file).unwrap();

    for line in contents.lines() {
        println!("{}", line);
    }

    let contents = fs::read(file).unwrap();

    println!("{:?}", contents);
}
