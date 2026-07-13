// Scenario:
// A program needs readable output for values, debug data, alignment, or decimal
// formatting.
//
// Thinking:
// Rust formatting macros let you choose display style without manually building
// every string.

pub fn run() {
    println!("\n7. Formatted output");

    let name = "Asha";
    let score = 42;
    let price = 19.995;

    println!("Display values: {} scored {}", name, score);
    println!("Debug tuple: {:?}", (name, score));
    println!("Pretty debug list: {:#?}", vec!["input", "output"]);
    println!("Price with two decimals: {:.2}", price);
    println!("Right aligned score: {:>5}", score);
}
