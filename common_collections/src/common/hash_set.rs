// Scenario:
// You need unique values, such as selected IDs, tags, or visited pages.
//
// Thinking:
// HashSet<T> is chosen when membership checks and uniqueness matter, but sorted
// order does not matter.

use std::collections::HashSet;

pub fn run() {
    println!("\n4. HashSet<T>");

    let mut selected_ids = HashSet::new();
    selected_ids.insert(1);
    selected_ids.insert(2);
    selected_ids.insert(2);

    println!("Selected IDs: {:?}", selected_ids);
    println!("Contains 2: {}", selected_ids.contains(&2));

    selected_ids.remove(&1);
    println!("After removing 1: {:?}", selected_ids);
}
