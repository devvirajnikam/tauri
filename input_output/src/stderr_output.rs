// Scenario:
// A program needs to separate normal output from error or diagnostic output.
//
// Thinking:
// println! writes to stdout. eprintln! writes to stderr. Keeping them separate is
// useful for CLI tools, logs, and shell redirection.

pub fn run() {
    println!("\n8. stdout and stderr");

    println!("This is normal output on stdout.");
    eprintln!("This is diagnostic output on stderr.");
}
