// Scenario:
// You need a queue where items are added at the back and removed from the front.
//
// Thinking:
// VecDeque<T> is chosen when both front and back operations should be efficient.

use std::collections::VecDeque;

pub fn run() {
    println!("\n5. VecDeque<T>");

    let mut queue = VecDeque::new();
    queue.push_back(String::from("first job"));
    queue.push_back(String::from("second job"));
    queue.push_front(String::from("urgent job"));

    println!("Queue: {:?}", queue);

    while let Some(job) = queue.pop_front() {
        println!("Processing: {}", job);
    }
}
