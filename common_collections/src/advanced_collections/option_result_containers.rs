// Scenario:
// Sometimes there is zero-or-one value, or either success data or error data.
//
// Thinking:
// Option<T> and Result<T, E> are enum containers. They are not collections like
// Vec, but they behave like small containers that force missing/error handling.

fn find_user_name(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Asha"))
    } else {
        None
    }
}

fn parse_quantity(input: &str) -> Result<u32, String> {
    input
        .parse::<u32>()
        .map_err(|error| format!("invalid quantity: {}", error))
}

pub fn run() {
    println!("\n22. Option<T> and Result<T, E> as containers");

    println!("User name: {:?}", find_user_name(1));
    println!(
        "Missing user fallback: {}",
        find_user_name(99).unwrap_or(String::from("Guest"))
    );
    println!("Parsed quantity: {:?}", parse_quantity("25"));
    println!("Bad quantity: {:?}", parse_quantity("abc"));
}
