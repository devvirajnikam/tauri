// Scenario:
// A form can have multiple validation errors at the same time, such as missing
// name and invalid email.
//
// Thinking:
// Use a struct for the input and a Vec<Enum> for the errors. The enum keeps each
// possible validation problem explicit and easy to match.

#[derive(Debug)]
struct SignupForm {
    name: String,
    email: String,
    age: u8,
}

#[derive(Debug)]
enum SignupError {
    NameRequired,
    InvalidEmail,
    TooYoung { minimum_age: u8 },
}

fn validate_signup(form: &SignupForm) -> Vec<SignupError> {
    let mut errors = Vec::new();

    if form.name.trim().is_empty() {
        errors.push(SignupError::NameRequired);
    }

    if !form.email.contains('@') {
        errors.push(SignupError::InvalidEmail);
    }

    if form.age < 18 {
        errors.push(SignupError::TooYoung { minimum_age: 18 });
    }

    errors
}

pub fn run() {
    println!("\n41. Form validation with Vec<Enum>");

    let form = SignupForm {
        name: String::from(""),
        email: String::from("wrong-email"),
        age: 15,
    };

    let errors = validate_signup(&form);

    for error in errors {
        match error {
            SignupError::NameRequired => println!("Name is required"),
            SignupError::InvalidEmail => println!("Email is invalid"),
            SignupError::TooYoung { minimum_age } => {
                println!("Age must be at least {}", minimum_age);
            }
        }
    }
}
