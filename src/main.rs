mod file_handler;

use std::io;
use std::path::Path;

fn main() {
    let mut input = String::new();
    println!("Hello, let's get it started");
    println!("Enter the file path");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim();
            let is_file_exist = is_file_exist(trimmed_input);
            if is_file_exist { 
                println!("File {} exists", input);
                file_handler::handle_file(input)
            }
            else { println!("File doesn't exist") }
        }
        Err(error) => {
            println!("Uncorrected path: {}", error)
        }
    }
    
}

fn is_file_exist(path: &str) -> bool {
    Path::new(path).exists()
}
