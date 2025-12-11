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

            // CHECK LETTER PATTERNS ONE AT A TIME

            let num_str: String = number.to_string();

            // POINTER A = 1
            // SPLIT STRING INTO TWO PARTS
            // PART 1 COMPARISION STRING
            // PART 2 DUPLICATE MATCH STRING
            // SPLIT PART 2 INTO SUBPARTS OF LENGTH EQUAL TO PART 1
            // MATCH EACH SUBPART WITH PART 1
            // IF ALL SUBPARTS MATCH PART 1, THEN INVALID NUMBER
            // ELSE VALID NUMBER

            let mut pointer: i32 = 1;
            while pointer < num_str.len() as i32 {
                let part1: &str = &num_str[0..pointer as usize];
                let part2: &str = &num_str[pointer as usize..];

                // EDGE CASE: PART 2 LENGTH NOT A MULTIPLE OF PART 1 LENGTH
                if part2.len() % part1.len() != 0 {
                    pointer += 1;
                    continue;
                }

                let mut all_match: bool = true;
                for i in (0..part2.len()).step_by(part1.len()) {
                    let subpart: &str = &part2[i..i + part1.len()];
                    if subpart != part1 {
                        all_match = false;
                        break;
                    }
                }

                if all_match {
                    invalid_sum += number as i64;
                    break;
                }

                pointer += 1;
            }

        }

    }

    println!("The sum of all invalid numbers is: {}", invalid_sum);
}