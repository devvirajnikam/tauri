fn find_user_name(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("John"))
    } else {
        None
    }
}

fn divide(left: i32, right: i32) -> Result<i32, String> {
    if right == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(left / right)
    }
}

pub fn run() {
    println!("\n6. Option and Result");

    match find_user_name(1) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    match find_user_name(99) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    match divide(10, 2) {
        Ok(answer) => println!("10 / 2 = {}", answer),
        Err(error) => println!("Error: {}", error),
    }

    match divide(10, 0) {
        Ok(answer) => println!("10 / 0 = {}", answer),
        Err(error) => println!("Error: {}", error),
    }
}
