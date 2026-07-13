mod panic_examples;
mod result_basics;
mod unwrap_expect;

pub fn run() {
    println!("\nRust error handling basics");

    result_basics::run();
    unwrap_expect::run();
    panic_examples::run();
}
