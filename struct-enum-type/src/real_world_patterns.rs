// Scenario:
// These modules show how structs and enums appear in real application code.
//
// Thinking:
// Beginner examples teach syntax. Real-world examples teach when to choose a
// struct, enum, trait, conversion, validation type, or state machine.

mod api_dto_mapping;
mod builder_pattern;
mod cache_entry;
mod collection_fields;
mod command_handler;
mod custom_display;
mod custom_error_enum;
mod domain_events;
mod external_service_result;
mod form_validation_errors;
mod from_into;
mod generic_trait_bounds;
mod money_currency;
mod optional_update_fields;
mod permission_roles;
mod phantom_state_types;
mod repository_trait;
mod search_query;
mod state_machine;
mod trait_objects;
mod try_from_validation;
mod typed_ids_newtypes;
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
    typed_ids_newtypes::run();
    api_dto_mapping::run();
    command_handler::run();
    domain_events::run();
    permission_roles::run();
    money_currency::run();
    repository_trait::run();
    optional_update_fields::run();
    phantom_state_types::run();
    cache_entry::run();
    form_validation_errors::run();
    search_query::run();
    external_service_result::run();
}
