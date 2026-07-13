// Scenario:
// You want to read one line from the terminal.
//
// Thinking:
// stdin().read_line(...) appends user input into a String and returns how many
// bytes were read.

use std::io;

pub fn run() {
    println!("\n1. Simple read_line");
    println!("Type any text and press Enter:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("You typed: {}", input.trim_end());
}
