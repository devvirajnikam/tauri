mod generic_repository;
mod generic_response;
mod generic_validator;
mod newtype_generic_id;
mod trait_bounds;
mod where_clause;

pub fn run() {
    println!("\nPractical generic patterns");

    trait_bounds::run();
    where_clause::run();
    generic_response::run();
    generic_validator::run();
    generic_repository::run();
    newtype_generic_id::run();
}
