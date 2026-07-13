// Scenario:
// Sometimes example code wants to crash immediately if a value is missing or bad.
//
// Thinking:
// unwrap and expect are convenient, but they panic on Err or None. They are fine
// for quick demos, tests, and impossible states, but not for normal user input.

pub fn run() {
    println!("\n2. unwrap and expect");

    let parsed_number = "42".parse::<i32>().expect("42 should be a valid number");
    println!("Parsed number with expect: {}", parsed_number);

    let maybe_name = Some("Asha");
    let name = maybe_name.unwrap();
    println!("Unwrapped name: {}", name);
}
