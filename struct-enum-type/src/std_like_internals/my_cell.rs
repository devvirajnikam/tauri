// Scenario:
// Sometimes code needs to change a small value even when it only has a shared
// reference to a wrapper.
//
// Thinking:
// Real Cell<T> uses interior mutability. This simplified version only supports
// Copy values and shows the get/set API shape using std::cell::Cell internally.

#[derive(Debug)]
struct MyCell<T: Copy> {
    value: std::cell::Cell<T>,
}

impl<T: Copy> MyCell<T> {
    fn new(value: T) -> MyCell<T> {
        MyCell {
            value: std::cell::Cell::new(value),
        }
    }

    fn get(&self) -> T {
        self.value.get()
    }

    fn set(&self, value: T) {
        self.value.set(value);
    }
}

pub fn run() {
    println!("\n55. Recreated Cell-like struct");

    let counter = MyCell::new(1);
    println!("Before set: {}", counter.get());

    counter.set(2);
    println!("After set through shared reference: {}", counter.get());
}
