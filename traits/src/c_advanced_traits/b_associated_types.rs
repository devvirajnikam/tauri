// Scenario:
// A trait has one main related type chosen by each implementation.
//
// Thinking:
// Associated types keep the output type attached to the implementation.

trait Loader {
    type Output;

    fn load(&self) -> Self::Output;
}

struct ConfigLoader;

impl Loader for ConfigLoader {
    type Output = String;

    fn load(&self) -> Self::Output {
        String::from("loaded config")
    }
}

pub fn run() {
    println!("\n11. Associated types");

    let loader = ConfigLoader;
    println!("{}", loader.load());
}
