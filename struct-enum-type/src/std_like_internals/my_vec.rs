// Scenario:
// A list needs to grow, read by index, and report how many items it contains.
//
// Thinking:
// Vec<T> is a struct in real Rust that manages pointer, length, and capacity.
// This learning version stores items inside the real Vec so we can focus on the
// public shape and behavior.

#[derive(Debug)]
struct MyVec<T> {
    items: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> MyVec<T> {
        MyVec { items: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

pub fn run() {
    println!("\n51. Recreated Vec-like struct");

    let mut numbers = MyVec::new();
    numbers.push(10);
    numbers.push(20);

    println!("{:?}", numbers);
    println!("Length: {}", numbers.len());
    println!("Index 1: {:?}", numbers.get(1));
}
