use std::env;
use std::fs;

pub fn day2() {
    // Challenge Part 1
    let contents: String = fetch_file();
    let parsed_reports = parse_reports(contents);
    let safe_reports = process_reports(parsed_reports);
    println!("The reports have been processed. Without the Problem Dampener, the number of safe reports are {safe_reports}.");
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

pub fn parse_reports(contents: String) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let mut paresed_numbers: Vec<i32> = Vec::new();

        for num in numbers {
            match num.parse::<i32>() {
                Ok(number) => {
                    paresed_numbers.push(number);
                }
                _ => {
                    println!("Skiping level: Could not parse numbers\n{num} in line {line}");
                    continue;
                }
            }
        }

        reports.push(paresed_numbers);
    }
    println!(
        "File successfully parsed! There are {} reports to process.",
        reports.len()
    );
    return reports;
}

fn process_reports(reports: Vec<Vec<i32>>) -> i32 {
    // A report is considered safe if all the levels are increasing or decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    let mut safe_reports = 0;
    for idx in 0..reports.len() {
        // Using & to pass variable by reference
        if process_level(&reports[idx], None) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}

fn process_level(line: &Vec<i32>, dampner: Option<bool>) -> bool {
    // Algorithm:
    // First navigate through each line without any removals. If valid, return true.
    // If invalid, stop. Remove elements from each line and pass it into the function recursively.
    // If a valid line is found, return true overall. Else, return false.
    let dampner_applied = dampner.unwrap_or(false);
    let mut is_safe = true;
    let mut is_increasing: Option<bool> = None;
    for idx in 0..line.len() - 1 {
        let current_level = line[idx];
        let next_level = line[idx + 1];
        let difference = (current_level - next_level).abs();

        // Check Condition 1: Difference between 1 and 3.
        if difference == 0 || difference > 3 {
            is_safe = false;
            break;
        }

        // Check Condition 2: Level is always increasing or decreasing.
        if is_increasing.is_none() {
            is_increasing = Some(current_level < next_level);
        } else {
            let current_direction = current_level < next_level;
            if current_direction != is_increasing.unwrap() {
                is_safe = false;
                break;
            }
        }
    }

    if is_safe {
        return true;
    } else if !dampner_applied {
        for idx in 0..line.len() {
            let agumented_line: Vec<i32> = line
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, val)| *val)
                .collect();
            if process_level(&agumented_line, Some(true)) {
                return true;
            }
        }
    }
    return false;
}
