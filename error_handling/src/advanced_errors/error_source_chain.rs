// Scenario:
// A high-level error should keep the lower-level cause for debugging.
//
// Thinking:
// Implementing std::error::Error with source() lets errors form a chain. This is
// useful when one layer wraps another layer's failure.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct ConfigError {
    field: String,
    source: ParseIntError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid config field '{}'", self.field)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_worker_count(input: &str) -> Result<u32, ConfigError> {
    input.parse::<u32>().map_err(|source| ConfigError {
        field: String::from("worker_count"),
        source,
    })
}

pub fn run() {
    println!("\n10. Error source chain");

    match parse_worker_count("abc") {
        Ok(count) => println!("Worker count: {}", count),
        Err(error) => {
            println!("Error: {}", error);
            if let Some(source) = error.source() {
                println!("Caused by: {}", source);
            }
        }
    }
}
