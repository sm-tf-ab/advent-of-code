use crate::scripts::fetch_file::fetch_file;

pub fn solve() {
    println!("Solving Day 3, Challenge 01!");

    let mut total_joltage: i32 = 0;

    // FETCH FILE CONTENTS
    let contents = fetch_file();

    // SPLIT STRING BY NEWLINE
    let lines: Vec<&str> = contents.lines().collect();

    for line in lines {
        // TRIM WHITESPACES
        let trimmed = line.trim();

        // EDGE CASES: SKIP EMPTY STRINGS
        if trimmed.is_empty() {
            continue;
        }

        // CREATE POINTERS
        // let mut chars: Vec<char> = trimmed.chars().collect();
        // let mut max_pair: i32 = format!("{}{}", pointer_a, pointer_b).parse().unwrap();
        // let mut max_pair_value:i32 = -9999;

        // ITERATE THROUGH THE STRING PARSING AT EACH STEP
        // FIND THE MAXIMUM TWO NUMBERS IN ORDER
        // for i in 0..chars.len() {
        //     for j in (i + 1)..chars.len() {
        //         let candidate_pair: i32 = format!("{}{}", chars[i], chars[j]).parse().unwrap();
        //         if candidate_pair > max_pair_value {
        //             max_pair_value = candidate_pair;
        //         }
        //     }
        // }

        let mut chars: Vec<char> = trimmed.chars().collect();
        let mut max_pair_value: i32 = -9999;

        // OPTIMIZED O(n) APPROACH
        // Track the maximum digit seen so far
        let mut max_first_digit: char = '0';
        
        for i in 0..chars.len() {

            // CONDITION ENSURES FIRST INDEX IS SKIPPED
            if i > 0 {
                // Form pair with max digit seen before current position
                let candidate_pair: i32 = format!("{}{}", max_first_digit, chars[i]).parse().unwrap();
                if candidate_pair > max_pair_value {
                    max_pair_value = candidate_pair;
                }
            }
            
            if chars[i] > max_first_digit {
                max_first_digit = chars[i];
            }
        }

        // ADD THE CONCATINATION OF THE TWO NUMBERS TO THE TOTAL JOLATGE
        total_joltage += max_pair_value;
    }

    println!("Total Joltage: {}", total_joltage);
}
