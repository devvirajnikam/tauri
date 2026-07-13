// Scenario:
// You need to repeatedly process the largest or highest-priority item first.
//
// Thinking:
// BinaryHeap<T> is chosen for priority queues. By default it returns the largest
// item first.

use std::collections::BinaryHeap;

pub fn run() {
    println!("\n8. BinaryHeap<T>");

    let mut priorities = BinaryHeap::new();
    priorities.push(2);
    priorities.push(5);
    priorities.push(1);

    while let Some(priority) = priorities.pop() {
        println!("Next priority: {}", priority);
    }
}
