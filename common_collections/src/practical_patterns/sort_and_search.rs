// Scenario:
// A report needs sorted values, and then the app needs to check if a value is
// present.
//
// Thinking:
// Sort a Vec<T> when you need ordered output. Use binary_search only after the
// Vec is sorted.

pub fn run() {
    println!("\n16. Sort and search");

    let mut amounts = vec![500, 100, 300, 200];
    amounts.sort();

    println!("Sorted amounts: {:?}", amounts);

    match amounts.binary_search(&300) {
        Ok(index) => println!("Found 300 at index {}", index),
        Err(index) => println!("300 could be inserted at index {}", index),
    }
}
