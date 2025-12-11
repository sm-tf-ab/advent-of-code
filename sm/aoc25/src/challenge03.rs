use crate::scripts::fetch_file::fetch_file;

#[allow(dead_code)]
pub fn solve() {
    println!("Solving Day 02, Challenge 01!");
    
    // INITIALIZE VARIABLES
    let mut invalid_sum: i64 = 0;
    
    // FETCH FILE CONTENTS
    let contents = fetch_file();
    
    // SPLIT STRING BY COMMA
    let parts: Vec<&str> = contents.split(',').collect();

    for part in parts {
        // TRIM WHITESPACES
        let trimmed = part.trim();

        // EDGE CASE: SKIP EMPTY STRINGS
        if trimmed.is_empty() {
            continue;
        }

        // SPLIT RANGE BY DASH
        let range_parts: Vec<&str> = trimmed.split('-').collect();

        // EDGE CASE: INVALID RANGE FORMAT
        if range_parts.len() != 2 {
            eprintln!("Skipping invalid range: {trimmed}");
            continue;
        }

        // PARSE START NUMBER
        let start: i64 = match range_parts[0].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Skipping invalid start number in range: {trimmed}");
                continue;
            }
        };

        // PARSE END NUMBER
        let end: i64 = match range_parts[1].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Skipping invalid end number in range: {trimmed}");
                continue;
            }
        };

        // EDGE CASE: START GREATER THAN END
        if start > end {
            eprintln!("Skipping range where start > end: {trimmed}");
            continue;
        }

        // SOME PATTERN OF NUMBERS REPEATED TWICE i.e., N.length % 2 == 0
        // 

        for number in start..=end {
            // EDGE CASE: SKIP NUMBERS STARTING WITH '0'
            if number.to_string().chars().next().unwrap() == '0' {
                continue;
            }

            let num_str = number.to_string();
            if num_str.chars().count() % 2 == 0 {
                let mid = num_str.chars().count() / 2;
                let num_parts: (&str, &str) = num_str.split_at(mid);
                if num_parts.0 == num_parts.1 {
                    invalid_sum += number as i64;
                }
            }
        }

    }

    println!("The sum of all invalid numbers is: {}", invalid_sum);
}