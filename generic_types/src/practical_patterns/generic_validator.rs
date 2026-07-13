// Scenario:
// Different validation rules should share one interface.
//
// Thinking:
// A generic trait lets each validator choose the input type it validates.

trait Validator<T> {
    fn validate(&self, value: &T) -> bool;
}

struct NonEmpty;

impl Validator<String> for NonEmpty {
    fn validate(&self, value: &String) -> bool {
        !value.trim().is_empty()
    }
}

struct Minimum;

impl Validator<i32> for Minimum {
    fn validate(&self, value: &i32) -> bool {
        *value >= 18
    }
}

pub fn run() {
    println!("\n8. Generic validator trait");

    let name_rule = NonEmpty;
    let age_rule = Minimum;

    println!("Valid name: {}", name_rule.validate(&String::from("Asha")));
    println!("Valid age: {}", age_rule.validate(&21));
}
