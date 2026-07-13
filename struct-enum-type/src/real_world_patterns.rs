// Scenario:
// These modules show how structs and enums appear in real application code.
//
// Thinking:
// Beginner examples teach syntax. Real-world examples teach when to choose a
// struct, enum, trait, conversion, validation type, or state machine.

mod builder_pattern;
mod collection_fields;
mod custom_display;
mod custom_error_enum;
mod from_into;
mod generic_trait_bounds;
mod state_machine;
mod trait_objects;
mod try_from_validation;
mod vec_patterns;

pub fn run() {
    println!("\nReal-world struct and enum patterns");

    collection_fields::run();
    vec_patterns::run();
    from_into::run();
    try_from_validation::run();
    custom_display::run();
    custom_error_enum::run();
    generic_trait_bounds::run();
    trait_objects::run();
    state_machine::run();
    builder_pattern::run();
}
