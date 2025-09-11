//! This module handles file I/O and command-line arguments.

/// Reads the file path from the second command-line argument and prints its lines.
pub fn read_file_from_args() {
    // `nth(0)` is the program name, `nth(1)` is the first argument.
    if let Some(file_name) = std::env::args().nth(1) {
        println!("Attempting to read file: {}", file_name);
        // `std::fs::read_to_string` returns a `Result`. We should handle it.
        match std::fs::read_to_string(file_name) {
            Ok(file_contents) => {
                println!("--- File Contents ---");
                file_contents.lines().for_each(|line| println!("{}", line));
                //test it with numbers.txt file
                println!("---check number or line---");
                file_contents.lines().for_each(|line| {
                    if let Ok(value) = line.parse::<usize>() {
                        println!("{}", value);
                    } else {
                        println!("Line not a number");
                    }
                });
                println!("--- End of File ---");
                process_file_lines(&file_contents);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    } else {
        println!("File I/O: No file name provided. Skipping.");
        println!("Usage: cargo run -- <filename>");
    }
}

/// Demonstrates processing file lines with iterators.
fn process_file_lines(contents: &str) {
    println!("\n--- Processing file with iterators ---");
    println!("(Skipping first 2 lines, then taking the next 2 even-indexed lines)");
    contents
        .lines()
        .enumerate() // Get (index, line)
        .filter(|(idx, _)| idx % 2 == 0) // Keep even-indexed lines
        .skip(2) // Skip the first 2 of the remaining lines
        .take(2) // Take the next 2
        .for_each(|(_, line)| println!("  - Processed line: {}", line));
    println!("\n--- Processing file with iterator filter map---");
    contents
        .lines()
        .filter_map(|line| line.parse::<usize>().ok()) //this is for numbers file
        .for_each(|line| println!("{}", line));
}
