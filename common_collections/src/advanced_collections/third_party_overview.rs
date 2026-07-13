// Scenario:
// Standard collections are not always the perfect fit for every production case.
//
// Thinking:
// Third-party crates add specialized collection behavior. This project does not
// depend on them, but it is useful to know when they are commonly considered.

pub fn run() {
    println!("\n30. Third-party collection overview");

    let examples = [
        (
            "indexmap",
            "HashMap-like lookup with stable insertion order",
        ),
        (
            "smallvec",
            "Vec-like storage that can avoid heap allocation for small lists",
        ),
        ("dashmap", "Concurrent HashMap-like access across threads"),
        ("slab", "Fast storage with stable integer keys"),
    ];

    for (crate_name, use_case) in examples {
        println!("{} -> {}", crate_name, use_case);
    }
}
