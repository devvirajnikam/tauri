// Scenario:
// You want to understand Rust's linked list collection, even though it is less
// common in normal app code.
//
// Thinking:
// LinkedList<T> is rarely the first choice. Vec or VecDeque is usually better,
// but LinkedList exists for cases that need list-like node operations.

use std::collections::LinkedList;

pub fn run() {
    println!("\n9. LinkedList<T>");

    let mut list = LinkedList::new();
    list.push_back("middle");
    list.push_front("start");
    list.push_back("end");

    for value in &list {
        println!("List value: {}", value);
    }
}
