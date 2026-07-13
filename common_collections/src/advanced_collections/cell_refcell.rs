// Scenario:
// A value needs to change through a shared reference in a controlled way.
//
// Thinking:
// Cell<T> is good for Copy values. RefCell<T> checks borrowing rules at runtime
// and is commonly paired with Rc<T> in single-threaded graph-like data.

use std::cell::{Cell, RefCell};

pub fn run() {
    println!("\n21. Cell<T> and RefCell<T>");

    let counter = Cell::new(1);
    counter.set(counter.get() + 1);
    println!("Cell counter: {}", counter.get());

    let names = RefCell::new(vec![String::from("Asha")]);
    names.borrow_mut().push(String::from("Ravi"));

    println!("RefCell names: {:?}", names.borrow());
}
