use std::env;
use std::fs;

pub fn day3() {
    //Example:
    //xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)) 
}

fn fetch_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    //read_to_string(...) reads a file as one continuous line of text including all the newline
    //characters. This is why the variable has a type `String` and not `Vec<String>`

    //.expect() is a way to handle `Result` types. In Rust a `Result` can be `Ok(value)` or `Err(error)`. The code is equivelent to:
    // let contents = match fs::read_to_string(file_path) {
    //  OK(value) => value
    //  Err(error) => panic!("Should have been able to read the file: {}", error)
    // };

    println!("File found!");
    return contents;
}

