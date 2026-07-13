// Scenario:
// A missing value should become a proper error message.
//
// Thinking:
// Option<T> only says present or missing. Result<T, E> is better when the caller
// needs to know why missing is a problem.

fn find_user_name(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Asha"))
    } else {
        None
    }
}

fn require_user_name(id: u32) -> Result<String, String> {
    find_user_name(id).ok_or_else(|| format!("user {} was not found", id))
}

pub fn run() {
    println!("\n4. Convert Option to Result");

    println!("Existing user: {:?}", require_user_name(1));
    println!("Missing user: {:?}", require_user_name(99));
}
