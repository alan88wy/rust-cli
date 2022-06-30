use std::env;

pub fn run() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments");
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    // getting the nth argument

    let arg2 = env::args().nth(2).unwrap();

    println!("argument 2 is {}", arg2);
}
