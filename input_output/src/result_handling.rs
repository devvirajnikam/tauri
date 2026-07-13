// Scenario:
// Reading from stdin can fail, so the program should handle success and failure.
//
// Thinking:
// read_line returns Result<usize, io::Error>. Matching the Result makes both
// paths explicit.

use std::io;

pub fn run() {
    println!("\n2. Handling read_line Result");
    println!("Type another line and press Enter:");

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    match result {
        Ok(bytes_read) => {
            println!("Read {} bytes", bytes_read);
            println!("Input without newline: {}", input.trim_end());
        }
        Err(error) => {
            println!("Failed to read input: {}", error);
        }
    }
}
