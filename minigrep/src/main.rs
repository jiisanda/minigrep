use std::{env, fs};

fn main() {

    // read any command line arguments passed to it and then collect the values into vector
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String =&args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text: \n {contents}");
}
