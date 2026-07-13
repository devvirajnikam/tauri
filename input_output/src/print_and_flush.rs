// Scenario:
// A program needs to show output without automatically moving to the next line,
// such as a prompt before input.
//
// Thinking:
// print! does not automatically flush stdout. Flushing is important when the
// user should see the prompt before the program waits.

use std::io::{self, Write};

pub fn run() {
    println!("\n6. print! and flush");

    print!("This text is printed without an automatic newline. ");

    if let Err(error) = io::stdout().flush() {
        println!("Could not flush stdout: {}", error);
        return;
    }

    println!("Now the line is completed.");
}
