use std::env;
use std::fs;

pub fn day1() {
    // CHALLENGE PART 1

    let args: Vec<String> = env::args().collect();
    
    // Explaination:
    // `.collect()`: Collects all values from received from `env::args()` and stores them in vector.

    let file_path = &args[1];

    // Search for file provided through Terminal Input - Useful for different Inputs for each
    // challenge.

    // Specify file by running `cargo run -- <args>` where `<args>` would be `input/day1.txt` (i.e., `cargo run -- input/day1.txt`). You can
    // specify as many arguments as possible by separating by spaces. You may access them using &args[n] starting at 1. 

    println!("Searching for file {file_path}.");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("File found.");

    // Create two Vectors to store the two columns of values.
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    // Explaination:
    // `Vector<T>`: A vector of elements with type `T` is growable.

    // Process each line of the file
    for line in contents.lines() {
        // Split each line
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Explaination:
        // `.split_whitespace()`: splits a str by any form of whitespace include spaces, tabs,and
        // newlines. Creates an iterator with the pointer pointing at the first value.
        // `.collect()`: Collects all values from the iterator made using `.split_whitespace()` and stores them in vector.

        // Check to ensure no line with more or less than 2 columns is processed.
        if numbers.len() != 2 {
            println!("Skipping Line: More or Less than 2 Columns\n{line}");
            continue;
        }

        // Parse strings to numbers and add to vectors
        match (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
            (Ok(n1), Ok(n2)) => {
                column1.push(n1);
                column2.push(n2);
            }
            _ => {
                println!("Skipping Line: Could not parse numbers\n{line}");
                continue;
            }
        }

        // Explaination:
        // `match()`: Switch statement to handle all cases. In this case, we want to only add the
        // values if they are parsed successfully into numbers. For all other cases, return a error
        // and continue.
        // `_`: Represents any other value.
    }

    // Sort the vectors in place
    column1.sort_by(|a, b| a.cmp(b)); 
    column2.sort_by(|a, b| a.cmp(b));

    // Explaination:
    // `sort_by`: Sorts a vector by comparing pair of values.
    // `|params| body`: This is a closure, an anonymous
    // `|a, b| a.cmp(b)`: Compares `a` to `b` in ascending order. Flipping them would look for
    // descending order.
    // Alternatively, we can do columnName.sort();

    // Calculate sum of differences
    let mut distance_score = 0;
    for i in 0..column1.len() {
        distance_score += (column1[i] - column2[i]).abs();
    }

    // Explaination: 
    // `0..column1.len()`: Generates a range from 0 to the length of vector.

    println!("Total Difference in Lists is {distance_score}");


    // CHALLENGE PART 2
    
    let mut similarity_score = 0;
    for i in 0..column1.len() {
        similarity_score += column1[i] * (column2.iter().filter(|&&x| x == column1[i]).count() as i32);
    }

    // Explaination:
    // `.iter()`: creates an iterator from the provided vector. It passes in values by reference.
    // `.filter()`: Filters values based on the function. Gets values as reference. Returns values
    // as reference. So in this case, its a reference of the reference of value.
    // `&&x`: Dereferences the value so filter can interact with the directly. We use two `&&`
    // since it has two reference layers.

    println!("Similarity Score between the Lists is {similarity_score}");
}
