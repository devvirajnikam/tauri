// Scenario:
// A function may call different APIs that return different error types.
//
// Thinking:
// Box<dyn std::error::Error> is useful for examples and app-level code where you
// do not need callers to match exact error variants.

use std::error::Error;

fn parse_and_double(input: &str) -> Result<u32, Box<dyn Error>> {
    let number = input.parse::<u32>()?;
    Ok(number * 2)
}

pub fn run() {
    println!("\n9. Box<dyn Error>");

    println!("Parsed and doubled: {:?}", parse_and_double("21"));
    println!("Failed parse: {:?}", parse_and_double("abc"));
}
