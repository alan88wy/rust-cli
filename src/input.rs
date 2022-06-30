use std::io;
pub fn run() {
    let mut buffer = String::new();
    println!("Enter a message...");
    let _ = io::stdin().read_line(&mut buffer);
    println!("Your input is {}", buffer);
}
