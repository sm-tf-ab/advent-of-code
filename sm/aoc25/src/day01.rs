use std::env;
use std::fs;

// Helper function to turn the dial given an amount, current value of pointer and password.
fn turn_dial(pointer: &mut i32, password: &mut i32, amt: i32) {
    *pointer = (*pointer + amt).rem_euclid(100);

    println!("Pointer moved to position: {}", *pointer);

    if *pointer == 0 {
        *password += 1;
    }
}

pub fn solve() {

    // CHALLENGE PART 1
    println!("Solving Day 01!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    

    // Search for file provided through Terminal Input
    println!("Searching for file {file_path}.");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("File found.");

    // INITIALIZE THE POINTER AND PASSWORD
    let mut pointer: i32 = 50;
    let mut password: i32 = 0;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() < 2 {
            println!("Skipping Line: Less than 2 characters\n{line}");
            continue;
        }
        let (dir, num) = line.split_at(1);
        let direction = dir.chars().next().unwrap();
        let value: i32 = match num.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Skipping Line: Could not parse number\n{line}");
                continue;
            }
        };

        match direction {
            'L' => {
                turn_dial(&mut pointer, &mut password, -value);
            }
            'R' => {
                turn_dial(&mut pointer, &mut password, value);
            }
            _ => eprintln!("Unknown direction '{direction}' in line: {line}"),
        }
    }

    println!("Final Pointer Position: {pointer}");
    println!("Final Password: {password}");
}
