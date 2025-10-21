use std::fs;
use std::error::Error;
use std::env;
// in success case we return unitype - basically nothing
// error case return anytype of Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    // using ? at end means if it returns error type error will automatically be returned
    let contents = fs::read_to_string(config.filename)?;

    // .expect wraps the fileread in ok variant, if err it will exit and err
    // .expect("Something went wrong reading the file.0");

    println!("With text \n {}", contents);
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
pub struct Config {
   pub query: String,
   pub filename: String,
   pub case_sensitive: bool,
}

impl Config {
    // new is a convention for naming constructor functions
    // returning Result type must return OK of type Config, or a str
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

    // query is 1 because arg 0 is binary path
    let query = args[1].clone();
    let filename = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

        #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}