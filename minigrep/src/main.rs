use std::{env, fs};

fn main() {

    // read any command line arguments passed to it and then collect the values into vector
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text: \n {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: &String = &args[1].clone();
    let file_path: &String = &args[2].clone();

    Config { query: query.to_string(), file_path: file_path.to_string() }
}