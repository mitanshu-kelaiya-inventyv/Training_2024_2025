use std::env;
use std::{error::Error, fs};

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(mut args:impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next(); //the first value in the return value of env::args is the name of the program.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        //     let query = args[1].clone();
        //     let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
            Ok(Config { query  , file_path, ignore_case })
        }
}


pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    // fs::read_to_string takes the file_path, opens that file, and returns a value of type std::io::Result<String> that contains the file’s contents.
    let contents = fs::read_to_string(config.file_path)?;
    //trait object Box<dyn Error>
    //Box<dyn Error> means the function will return a type that implements the Error trait,
    // println!("{contents}");
    if config.ignore_case == true{
        for line in search_case_insensitive(&config.query, &contents){
            println!("{line}");
        }
    }
    else{
        for line in search(&config.query, &contents){
        println!("{line}");
    }
}

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick threee.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
Safe, Fast, Productive.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut results = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }

    contents.lines().filter(|line| line.contains(query)).collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut results = Vec::new();
    // let query = query.to_lowercase();

    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);
    //     }
    // }

    contents.lines().filter(|line|line.to_lowercase().contains(&query.to_lowercase())).collect()

    // results
}