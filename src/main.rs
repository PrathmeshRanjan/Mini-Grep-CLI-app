#![allow(non_snake_case)]

use std::env; //To read from the command line

use MiniGrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for: {}", config.query); //Displays the word we're searching for
    println!("In file: {}", config.filename); // Displays all of the content of the file we're searching in

    MiniGrep::run(config);
} 