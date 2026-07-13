// Scenario:
// A function is generic, but it needs to print the value.
//
// Thinking:
// Trait bounds say what behavior the generic type must support.

use std::fmt::Display;

fn print_twice<T: Display>(value: T) {
    println!("First: {}", value);
    println!("Second: {}", value);
}

pub fn run() {
    println!("\n5. Trait bounds");

    print_twice("generic text");
    print_twice(99);
}
