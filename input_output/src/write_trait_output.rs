// Scenario:
// Output should be written to any destination, not only directly to the terminal.
//
// Thinking:
// The Write trait lets the same logic write to stdout, files, buffers, or tests.

use std::io::{self, Write};

fn write_report(mut writer: impl Write, title: &str, count: usize) -> io::Result<()> {
    writeln!(writer, "Report: {}", title)?;
    writeln!(writer, "Count: {}", count)?;
    Ok(())
}

pub fn run() {
    println!("\n9. Write trait output");

    let mut buffer = Vec::new();

    match write_report(&mut buffer, "input-output", 3) {
        Ok(()) => {
            let text = String::from_utf8_lossy(&buffer);
            println!("Buffered report:\n{}", text);
        }
        Err(error) => println!("Could not write report: {}", error),
    }
}
