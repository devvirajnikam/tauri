// Scenario:
// A command-line program needs to keep reading commands until the user exits.
//
// Thinking:
// A loop is useful for repeated input. match is useful for deciding what each
// command should do.

use std::io::{self, Write};

pub fn run() {
    println!("\n5. Small command loop");
    println!("Commands: help, status, exit");

    loop {
        print!("command> ");

        if let Err(error) = io::stdout().flush() {
            println!("Could not flush prompt: {}", error);
            break;
        }

        let mut command = String::new();

        if let Err(error) = io::stdin().read_line(&mut command) {
            println!("Could not read command: {}", error);
            break;
        }

        match command.trim() {
            "help" => println!("Available commands: help, status, exit"),
            "status" => println!("Everything is running"),
            "exit" => {
                println!("Goodbye");
                break;
            }
            "" => println!("Please type a command"),
            unknown => println!("Unknown command: {}", unknown),
        }
    }
}
