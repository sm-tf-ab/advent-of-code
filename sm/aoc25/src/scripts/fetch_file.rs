use std::env;
use std::fs;

/// Read the file path passed as the first CLI argument and return its contents.
pub fn fetch_file() -> String {
    println!("Searching for file.");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("File found.");

    contents
}