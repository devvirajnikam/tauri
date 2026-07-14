// Scenario:
// Most implementations should share behavior, but each type can still override
// it when needed.
//
// Thinking:
// Default trait methods reduce repeated code while preserving customization.

trait Greet {
    fn name(&self) -> &str;

    fn greet(&self) -> String {
        format!("Hello, {}", self.name())
    }
}

struct User {
    name: String,
}

impl Greet for User {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn run() {
    println!("\n2. Default trait methods");

    let user = User {
        name: String::from("Ravi"),
    };

    println!("{}", user.greet());
}
