use std::env;

fn main() {

    // read any command line arguments passed to it and then collect the values into vector
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String =&args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
