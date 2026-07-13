// Scenario:
// You need key-value lookup, but you also need keys to stay sorted.
//
// Thinking:
// BTreeMap<K, V> is chosen when deterministic sorted key order matters.

use std::collections::BTreeMap;

pub fn run() {
    println!("\n6. BTreeMap<K, V>");

    let mut sales_by_month = BTreeMap::new();
    sales_by_month.insert(String::from("March"), 300);
    sales_by_month.insert(String::from("January"), 100);
    sales_by_month.insert(String::from("February"), 200);

    for (month, sales) in &sales_by_month {
        println!("{} -> {}", month, sales);
    }
}
