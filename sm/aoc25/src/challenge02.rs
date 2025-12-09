use std::env;
use std::fs;

pub fn solve() {
    // CHALLENGE DAY 1 PART 2
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
                // L150 -> 150 - 50 = 100 -> 100 % 100 = 0
                // L1000 -> 1000 - 50 = 950 -> 950 / 100 = 9.5 -> floor(9.5) = 9 + 1 = 10
                // L30 -> 30 - 50 = -20 -> -20 / 100 -0.2 -> floor(-0.2) = -1 + 1 = 0
                if(pointer != 0) {
                    password += ((value - pointer) as f64 / 100.0).floor() as i32 + 1;
                } else {
                    password += (value as f64 / 100.0).floor() as i32;
                }
                pointer = (pointer - value).rem_euclid(100);
            }
            'R' => {
                password += ((pointer + value) as f64 / 100.0).floor() as i32;
                pointer = (pointer + value).rem_euclid(100);
                // 50 + 50 = 100. Therfore, password += (50 + 50)/100 = 1
                //50 + 150 = 200. Therefore, password += (50 + 150)/100 = 2
            }
            _ => eprintln!("Unknown direction '{direction}' in line: {line}"),
        }
    }

    println!("Final Pointer Position: {pointer}");
    println!("Final Password: {password}");
}
