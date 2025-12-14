use crate::scripts::fetch_file::fetch_file;
use num_bigint::BigInt;

fn best_12_digits(line: &str) -> BigInt {
    let mut stack: Vec<char> = Vec::with_capacity(12);
    let mut drops = line.len().saturating_sub(12);

    for ch in line.chars() {
        while drops > 0 && !stack.is_empty() && *stack.last().unwrap() < ch {
            stack.pop();
            drops -= 1;
        }
        stack.push(ch);
    }

    stack.truncate(12);
    stack.iter().collect::<String>().parse::<BigInt>().unwrap()
}

pub fn solve() {
    println!("Solving Day 3, Challenge 02!");

    let mut total_joltage = BigInt::from(0);

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

        total_joltage += best_12_digits(trimmed);
    }

    println!("Total Joltage: {}", total_joltage);
}
