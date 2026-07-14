// Scenario:
// A trait should require another trait's behavior.
//
// Thinking:
// A supertrait says implementers must also implement another trait.

use std::fmt::Display;

trait PrettyPrint: Display {
    fn pretty_print(&self) {
        println!("*** {} ***", self);
    }
}

struct Title(String);

impl Display for Title {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl PrettyPrint for Title {}

pub fn run() {
    println!("\n12. Supertraits");

    let title = Title(String::from("Traits"));
    title.pretty_print();
}
