// Scenario:
// A function has multiple generic rules and the signature becomes hard to read.
//
// Thinking:
// where clauses move trait bounds below the function signature for readability.

use std::fmt::{Debug, Display};

fn describe_pair<T, U>(left: T, right: U)
where
    T: Display + Debug,
    U: Display + Debug,
{
    println!("Left display: {}, debug: {:?}", left, left);
    println!("Right display: {}, debug: {:?}", right, right);
}

pub fn run() {
    println!("\n6. where clauses");

    describe_pair("age", 30);
}
