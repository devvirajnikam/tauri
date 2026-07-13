// Scenario:
// Multiple parts of a program need shared ownership of the same value.
//
// Thinking:
// Rc<T> is for single-threaded shared ownership. Arc<T> is for thread-safe shared
// ownership. Weak<T> observes shared data without keeping it alive.

use std::rc::{Rc, Weak};
use std::sync::Arc;

pub fn run() {
    println!("\n20. Rc<T>, Arc<T>, and Weak<T>");

    let shared_name = Rc::new(String::from("shared user"));
    let first_owner = Rc::clone(&shared_name);
    let weak_owner: Weak<String> = Rc::downgrade(&shared_name);

    println!("Rc owners: {}", Rc::strong_count(&shared_name));
    println!("First owner: {}", first_owner);
    println!("Weak can upgrade: {}", weak_owner.upgrade().is_some());

    let shared_config = Arc::new(String::from("thread-safe config"));
    let second_config_owner = Arc::clone(&shared_config);

    println!("Arc value: {}", second_config_owner);
}
