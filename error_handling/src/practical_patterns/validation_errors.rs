// Scenario:
// A form can have more than one validation problem at the same time.
//
// Thinking:
// Result<T, Vec<E>> is useful when you want to return all validation errors
// instead of stopping at the first one.

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
    TooYoung,
}

fn validate_signup(form: SignupForm) -> Result<SignupForm, Vec<SignupError>> {
    let mut errors = Vec::new();

    if form.name.trim().is_empty() {
        errors.push(SignupError::NameRequired);
    }

    if !form.email.contains('@') {
        errors.push(SignupError::InvalidEmail);
    }

    if form.age < 18 {
        errors.push(SignupError::TooYoung);
    }

    if errors.is_empty() {
        Ok(form)
    } else {
        Err(errors)
    }
}

pub fn run() {
    println!("\n8. Collect multiple validation errors");

    let form = SignupForm {
        name: String::from(""),
        email: String::from("wrong-email"),
        age: 15,
    };

    println!("Validation result: {:?}", validate_signup(form));
}
