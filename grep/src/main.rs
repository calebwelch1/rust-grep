use std::env;
use std::fs;
use std::process;
fn main() {
    // env::args gives us args passed to program, collect makes them a collection
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else in OK case returns value in OK
    // in error case will execute closure with err
    let config: Config = Config::new(&args)
    .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });



    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("{:?}", args);

    // .expect wraps the fileread in ok variant, if err it will exit and err
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file.0");

    println!("With text \n {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // new is a convention for naming constructor functions
    // returning Result type must return OK of type Config, or a str
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

    // query is 1 because arg 0 is binary path
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
}
}

