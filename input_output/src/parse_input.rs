// Scenario:
// Terminal input is always text, but the program may need a number.
//
// Thinking:
// First read a String, then trim it, then parse it into the target type. Parsing
// also returns Result because the user may type invalid text.

use std::io;

pub fn run() {
    println!("\n3. Parse typed input");
    println!("Type a number and press Enter:");

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("Failed to read input: {}", error);
        return;
    }

    match input.trim().parse::<i32>() {
        Ok(number) => println!("Number doubled: {}", number * 2),
        Err(error) => println!("That was not a valid i32: {}", error),
    }
}
