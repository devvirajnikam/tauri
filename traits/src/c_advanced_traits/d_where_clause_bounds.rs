// Scenario:
// A function has multiple trait requirements and the signature becomes noisy.
//
// Thinking:
// where clauses make complex trait bounds easier to read.

use std::fmt::{Debug, Display};

fn print_debug_and_display<T>(value: T)
where
    T: Debug + Display,
{
    println!("Display: {}", value);
    println!("Debug: {:?}", value);
}

pub fn run() {
    println!("\n13. where clauses with traits");

    print_debug_and_display(42);
}
