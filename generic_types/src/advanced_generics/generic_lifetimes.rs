// Scenario:
// A generic struct stores borrowed data.
//
// Thinking:
// Lifetimes can be combined with generic types to describe how long borrowed
// generic data must remain valid.

#[derive(Debug)]
struct BorrowedValue<'a, T> {
    value: &'a T,
}

pub fn run() {
    println!("\n13. Generics with lifetimes");

    let number = 42;
    let borrowed = BorrowedValue { value: &number };

    println!("Borrowed value: {:?}", borrowed);
    println!("Borrowed raw value: {}", borrowed.value);
}
