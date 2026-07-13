// Scenario:
// Jobs should be processed in the same order they were added.
//
// Thinking:
// VecDeque<T> is a good queue because push_back and pop_front are efficient.

use std::collections::VecDeque;

pub fn run() {
    println!("\n14. Queue processing");

    let mut jobs = VecDeque::from([
        String::from("send email"),
        String::from("generate invoice"),
        String::from("sync stock"),
    ]);

    while let Some(job) = jobs.pop_front() {
        println!("Completed job: {}", job);
    }
}
