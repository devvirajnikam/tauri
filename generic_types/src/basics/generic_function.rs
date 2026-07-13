// Scenario:
// Two functions do the same logic for different types.
//
// Thinking:
// A generic function lets one function work with many types, as long as the
// function body only uses behavior that those types support.

fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

pub fn run() {
    println!("\n1. Generic function");

    let numbers = [10, 20, 30];
    let names = ["Asha", "Ravi", "Nina"];

    println!("First number: {:?}", first(&numbers));
    println!("First name: {:?}", first(&names));
}
