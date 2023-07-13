use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query: &String = &args[1].clone();
        let file_path: &String = &args[2].clone();

        Ok(Config { query: query.to_string(), file_path: file_path.to_string() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n {contents}");

    Ok(())
}