// Scenario:
// Creating a user can fail for different business reasons.
//
// Thinking:
// Use an enum for errors when there are multiple known failure cases. Each
// variant can carry only the data needed for that specific error.

use std::fmt;

#[derive(Debug)]
enum CreateUserError {
    EmptyName,
    AgeTooLow { minimum_age: u8 },
}

impl fmt::Display for CreateUserError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateUserError::EmptyName => write!(formatter, "name cannot be empty"),
            CreateUserError::AgeTooLow { minimum_age } => {
                write!(formatter, "age must be at least {}", minimum_age)
            }
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn create_user(name: String, age: u8) -> Result<User, CreateUserError> {
    if name.trim().is_empty() {
        return Err(CreateUserError::EmptyName);
    }

    if age < 18 {
        return Err(CreateUserError::AgeTooLow { minimum_age: 18 });
    }

    Ok(User { name, age })
}

pub fn run() {
    println!("\n26. Custom error enums");

    match create_user(String::from("Asha"), 25) {
        Ok(user) => println!(
            "Created user: {:?}, name={}, age={}",
            user, user.name, user.age
        ),
        Err(error) => println!("Error: {}", error),
    }

    match create_user(String::from(""), 12) {
        Ok(user) => println!(
            "Created user: {:?}, name={}, age={}",
            user, user.name, user.age
        ),
        Err(error) => println!("Error: {}", error),
    }
}
