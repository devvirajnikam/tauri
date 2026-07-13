// Scenario:
// Sometimes a value should live on the heap, while the variable stores a small
// handle to it.
//
// Thinking:
// Box<T> is a smart pointer struct in real Rust. This simplified version wraps T
// so you can see the "one value inside a wrapper" idea.

#[derive(Debug)]
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn into_inner(self) -> T {
        self.value
    }
}

pub fn run() {
    println!("\n50. Recreated Box-like struct");

    let boxed_name = MyBox::new(String::from("boxed value"));
    println!("Box wrapper: {:?}", boxed_name);
    println!("Borrow inner value: {}", boxed_name.get());
    println!("Move inner value: {}", boxed_name.into_inner());
}
