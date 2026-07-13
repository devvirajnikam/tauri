mod associated_functions;
mod derived_traits;
mod destructuring;
mod if_while_let;
mod lifetimes;
mod nested_types;
mod option_result_helpers;
mod pattern_guards;
mod recursive_enums;
mod self_methods;
mod tuple_unit_structs;
mod visibility;

pub fn run() {
    println!("\nAdvanced struct and enum topics");

    tuple_unit_structs::run();
    destructuring::run();
    if_while_let::run();
    self_methods::run();
    associated_functions::run();
    visibility::run();
    nested_types::run();
    lifetimes::run();
    derived_traits::run();
    recursive_enums::run();
    pattern_guards::run();
    option_result_helpers::run();
}
