// Scenario:
// A calculation can succeed or fail depending on the input.
//
// Thinking:
// Result<T, E> is the main Rust type for recoverable errors. Ok(T) carries a
// success value, and Err(E) carries error information.

fn divide(left: i32, right: i32) -> Result<i32, String> {
    if right == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(left / right)
    }
}

pub fn run() {
    println!("\n1. Result<T, E> basics");

    match divide(10, 2) {
        Ok(answer) => println!("10 / 2 = {}", answer),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10, 0) {
        Ok(answer) => println!("10 / 0 = {}", answer),
        Err(error) => println!("Error: {}", error),
    }
}
