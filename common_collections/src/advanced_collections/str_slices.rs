// Scenario:
// You need to read text without taking ownership of it.
//
// Thinking:
// &str is a borrowed view into UTF-8 text. It is collection-like because it lets
// you inspect text as bytes, chars, words, or slices.

fn print_label(label: &str) {
    println!("Label: {}", label);
}

pub fn run() {
    println!("\n17. &str string slices");

    let owned = String::from("common collections");
    let borrowed: &str = &owned;
    let first_word = &owned[0..6];

    print_label(borrowed);
    println!("First word slice: {}", first_word);
    println!("Character count: {}", borrowed.chars().count());
}
