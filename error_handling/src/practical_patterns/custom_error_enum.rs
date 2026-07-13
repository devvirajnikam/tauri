// Scenario:
// A function can fail for several known business reasons.
//
// Thinking:
// A custom error enum is better than String when callers may need to match on
// the exact failure type.

use std::fmt;

#[derive(Debug)]
enum CreateAccountError {
    EmptyName,
    AgeTooLow { minimum_age: u8 },
}

impl fmt::Display for CreateAccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateAccountError::EmptyName => write!(formatter, "name cannot be empty"),
            CreateAccountError::AgeTooLow { minimum_age } => {
                write!(formatter, "age must be at least {}", minimum_age)
            }
        }
    }
}

fn create_account(name: &str, age: u8) -> Result<String, CreateAccountError> {
    if name.trim().is_empty() {
        return Err(CreateAccountError::EmptyName);
    }

    if age < 18 {
        return Err(CreateAccountError::AgeTooLow { minimum_age: 18 });
    }

    Ok(format!("account created for {}", name))
}

pub fn run() {
    println!("\n7. Custom error enum");

    match create_account("Asha", 25) {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Error: {}", error),
    }

    match create_account("", 12) {
        Ok(message) => println!("{}", message),
        Err(error) => println!("Error: {}", error),
    }
}
