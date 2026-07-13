mod advanced_topics;
mod basic_enum;
mod basic_struct;
mod enum_with_data;
mod generics;
mod impl_methods;
mod match_examples;
mod option_result;
mod real_world_patterns;
mod std_like_internals;
mod traits;

fn main() {
    basic_struct::run();
    impl_methods::run();
    basic_enum::run();
    enum_with_data::run();
    match_examples::run();
    option_result::run();
    generics::run();
    traits::run();
    advanced_topics::run();
    real_world_patterns::run();
    std_like_internals::run();
}
