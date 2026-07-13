// Scenario:
// You need to count how many times each word, item, or status appears.
//
// Thinking:
// HashMap<T, usize> is the standard frequency-counter shape.

use std::collections::HashMap;

pub fn run() {
    println!("\n11. Frequency counter");

    let words = ["rust", "vec", "rust", "map", "vec", "rust"];
    let mut counts = HashMap::new();

    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", counts);
}
