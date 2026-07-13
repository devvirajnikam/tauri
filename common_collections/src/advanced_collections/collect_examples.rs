// Scenario:
// Iterator output needs to become a concrete collection.
//
// Thinking:
// collect() is generic. The target type decides whether output becomes Vec,
// HashMap, HashSet, String, or another FromIterator type.

use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("\n25. collect::<...>() examples");

    let numbers = [1, 2, 2, 3];
    let number_vec = numbers.iter().copied().collect::<Vec<i32>>();
    let number_set = numbers.iter().copied().collect::<HashSet<i32>>();

    let pairs = [(1, "one"), (2, "two")];
    let number_words = pairs.into_iter().collect::<HashMap<i32, &str>>();

    let text = ['R', 'u', 's', 't'].into_iter().collect::<String>();

    println!("Vec: {:?}", number_vec);
    println!("HashSet: {:?}", number_set);
    println!("HashMap: {:?}", number_words);
    println!("String: {}", text);
}
