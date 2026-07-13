// Scenario:
// A trait has one main output type chosen by each implementation.
//
// Thinking:
// Associated types are often cleaner than generic traits when the implementer
// should decide the related type once.

trait Loader {
    type Item;

    fn load(&self) -> Self::Item;
}

struct UserLoader;

impl Loader for UserLoader {
    type Item = String;

    fn load(&self) -> Self::Item {
        String::from("loaded user")
    }
}

pub fn run() {
    println!("\n14. Associated types");

    let loader = UserLoader;
    println!("Loaded item: {}", loader.load());
}
