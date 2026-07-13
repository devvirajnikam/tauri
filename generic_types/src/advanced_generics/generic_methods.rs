// Scenario:
// A non-generic type has a method that should accept many input types.
//
// Thinking:
// Methods can have their own generic parameters even when the struct is not
// generic.

struct Logger;

impl Logger {
    fn log<T: std::fmt::Display>(&self, value: T) {
        println!("Log: {}", value);
    }
}

pub fn run() {
    println!("\n12. Generic methods");

    let logger = Logger;
    logger.log("started");
    logger.log(200);
}
