use std::char;

use crate::scripts::fetch_file::fetch_file;

fn check_accessibility(lines: &Vec<Vec<char>>, line_idx: usize, char_idx: usize) -> bool {
    let mut adjecent_count = 0;

    if line_idx > 0 {
        if let Some(line) = lines.get(line_idx - 1) {
            for c in [-1, 0, 1] {
                if let Some(col) = char_idx.checked_add_signed(c as isize) {
                    if lines[line_idx - 1].get(col) == Some(&'@') {
                        adjecent_count += 1;
                    }
                }
            }

            if adjecent_count >= 4 {
                return false;
            }
        }
    }

    if let Some(line) = lines.get(line_idx) {
        for c in [-1, 1] {
            if let Some(col) = char_idx.checked_add_signed(c as isize) {
                if lines[line_idx].get(col) == Some(&'@') {
                    adjecent_count += 1;
                }
            }
        }

        if adjecent_count >= 4 {
            return false;
        }
    }

    if let Some(line) = lines.get(line_idx + 1) {
        for c in [-1, 0, 1] {
            if let Some(col) = char_idx.checked_add_signed(c as isize) {
                if lines[line_idx + 1].get(col) == Some(&'@') {
                    adjecent_count += 1;
                }
            }
        }

        if adjecent_count >= 4 {
            return false;
        }
    }

    if adjecent_count >= 4 {
        return false;
    }

    return true;
}

pub fn solve() {
    println!("Solving Day 4, Challenge 02!");

    // FETCH CHALLANGE INPUT FILE
    let contents = fetch_file();

    // println!("File Contents: {}", contents);

    let mut lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut accesible_rolls = 0;
    let mut found = true;

    while found {
        found = false;
        for mut i in 0..lines.len() {
            for char in 0..lines[i].len() {
                if (lines[i][char] == '@') {
                    let accessible = check_accessibility(&lines, i, char);

                    if accessible {
                        accesible_rolls += 1;
                        lines[i][char] = '.';
                        found = true;
                    }
                }
            }
        }
    }

    println!("Total Accessible Rolls: {}", accesible_rolls);
}
