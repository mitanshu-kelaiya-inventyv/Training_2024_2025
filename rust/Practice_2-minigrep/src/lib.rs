use std::{error::Error, fs};

pub struct Config{
    pub query: String,
    pub file_path: String,
}

impl Config{
    pub fn build(args:&[String]) -> Result<Config, &'static str>{

            if args.len() < 3{
                return Err("Not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();
        
            Ok(Config { query  , file_path })
        }
}


pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    // fs::read_to_string takes the file_path, opens that file, and returns a value of type std::io::Result<String> that contains the file’s contents.
    let contents = fs::read_to_string(config.file_path)?;
    //trait object Box<dyn Error>
    //Box<dyn Error> means the function will return a type that implements the Error trait,
    // println!("{contents}");

    for line in search(&config.query, &contents){
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick threee.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}