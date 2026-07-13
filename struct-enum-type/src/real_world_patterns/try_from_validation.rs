// Scenario:
// User input may look like a normal value, but it still needs validation before
// the app should trust it.
//
// Thinking:
// Use TryFrom when creating a type can fail. Here Age protects the rest of the
// program from invalid age values.

#[derive(Debug)]
struct Age(u8);

impl TryFrom<u8> for Age {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 120 {
            Ok(Age(value))
        } else {
            Err(format!("{} is not a valid age", value))
        }
    }
}

pub fn run() {
    println!("\n24. TryFrom and TryInto validation");

    let valid_age = Age::try_from(30);
    let invalid_age: Result<Age, String> = 200_u8.try_into();

    match valid_age {
        Ok(age) => println!("Valid age: {:?}, value={}", age, age.0),
        Err(error) => println!("Error: {}", error),
    }

    match invalid_age {
        Ok(age) => println!("Valid age: {:?}, value={}", age, age.0),
        Err(error) => println!("Error: {}", error),
    }
}
