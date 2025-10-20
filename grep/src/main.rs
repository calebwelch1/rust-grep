use std::env;
use std::fs;

fn main() {
    // env::args gives us args passed to program, collect makes them a collection
    let args: Vec<String> = env::args().collect();

    // query is 1 because arg 0 is binary path
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}", query);
    println!("In file: {}", filename);
    println!("{:?}", args);

    // .expect wraps the fileread in ok variant, if err it will exit and err
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.0");

    println!("With text \n {}", contents);
}
