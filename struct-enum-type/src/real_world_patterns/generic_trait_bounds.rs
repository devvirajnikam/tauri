// Scenario:
// A response wrapper should work with many data types, but a print function only
// works when the data knows how to display itself.
//
// Thinking:
// Generics make structs reusable. Trait bounds add rules to those generics, such
// as "T must implement Display".

use std::fmt::Display;

#[derive(Debug)]
struct ApiResponse<T> {
    data: T,
}

impl<T: Display> ApiResponse<T> {
    fn print(&self) {
        println!("Response data: {}", self.data);
    }
}

fn print_twice<T: Display>(value: T) {
    println!("First: {}", value);
    println!("Second: {}", value);
}

pub fn run() {
    println!("\n27. Generic trait bounds");

    let response = ApiResponse { data: 42 };
    response.print();

    print_twice("hello");
}
