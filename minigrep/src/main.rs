use std::{env, fs, process, error::Error};

fn main() {

    // read any command line arguments passed to it and then collect the values into vector
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query: &String = &args[1].clone();
        let file_path: &String = &args[2].clone();

        Ok(Config { query: query.to_string(), file_path: file_path.to_string() })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n {contents}");

    Ok(())
}