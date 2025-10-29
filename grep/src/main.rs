use std::env;
use std::process;

use grep::Config;
fn main() {
    // env::args gives us args passed to program, collect makes them a collection
    // let args: Vec<String> = env::args().collect();
    // // unwrap_or_else in OK case returns value in OK
    // // in error case will execute closure with err
    // let config: Config = Config::new(&args)
    let config: Config = Config::new(env::args())
    .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });



    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("{:?}", args);

    // need to handle if this does not return OK and returns Error
    // if let err variant ... print error and exit
    if let Err(e) = grep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

