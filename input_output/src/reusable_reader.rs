// Scenario:
// Multiple parts of the app need to ask for input.
//
// Thinking:
// A helper function avoids repeating String creation, read_line, trimming, and
// error mapping everywhere.

use std::io::{self, Write};

fn read_required_line(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn run() {
    println!("\n4. Reusable input helper");

    match read_required_line("Enter your name: ") {
        Ok(name) if name.is_empty() => println!("Name was empty"),
        Ok(name) => println!("Hello, {}", name),
        Err(error) => println!("Could not read name: {}", error),
    }
}
