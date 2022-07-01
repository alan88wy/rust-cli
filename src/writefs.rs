use std::env;
use std::fs;

pub fn run() {
    let mut file = String::from("test.md");

    if env::args().len() > 1 {
        file = env::args().nth(1).unwrap();
    }

    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon\n");
    speech.push_str("and do the other thing\n");

    let _ = fs::write(file, speech);
}
