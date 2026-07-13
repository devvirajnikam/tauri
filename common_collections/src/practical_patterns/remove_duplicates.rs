// Scenario:
// A list may contain repeated values, but you need each value only once.
//
// Thinking:
// HashSet<T> removes duplicates. Convert back to Vec<T> if the next step expects
// a list.

use std::collections::HashSet;

pub fn run() {
    println!("\n15. Remove duplicates");

    let ids = vec![1, 2, 2, 3, 1, 4];
    let unique_ids: HashSet<u32> = ids.into_iter().collect();
    let mut unique_list: Vec<u32> = unique_ids.into_iter().collect();

    unique_list.sort();
    println!("Unique sorted IDs: {:?}", unique_list);
}
