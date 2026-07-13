// Scenario:
// Production Rust apps often use crates to reduce error boilerplate.
//
// Thinking:
// thiserror is commonly used for library/domain error enums. anyhow is commonly
// used for app-level error reporting where exact matching is less important.

pub fn run() {
    println!("\n13. thiserror and anyhow overview");

    let crates = [
        ("thiserror", "derive Error for custom error enums"),
        ("anyhow", "easy app-level error context and reporting"),
        ("eyre", "another app-level error reporting option"),
    ];

    for (crate_name, use_case) in crates {
        println!("{} -> {}", crate_name, use_case);
    }
}
