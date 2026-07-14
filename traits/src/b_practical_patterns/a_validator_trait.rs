// Scenario:
// Validation rules should be reusable and swappable.
//
// Thinking:
// A trait lets every validation rule expose the same validate method.

trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), String>;
}

struct NonEmpty;

impl Validator<String> for NonEmpty {
    fn validate(&self, value: &String) -> Result<(), String> {
        if value.trim().is_empty() {
            Err(String::from("value cannot be empty"))
        } else {
            Ok(())
        }
    }
}

pub fn run() {
    println!("\n5. Validator trait");

    let validator = NonEmpty;

    println!(
        "Valid name: {:?}",
        validator.validate(&String::from("Asha"))
    );
    println!("Empty name: {:?}", validator.validate(&String::from("")));
}
