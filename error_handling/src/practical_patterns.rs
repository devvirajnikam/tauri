mod custom_error_enum;
mod map_err_examples;
mod option_to_result;
mod question_mark;
mod validation_errors;

pub fn run() {
    println!("\nPractical error handling patterns");

    option_to_result::run();
    question_mark::run();
    map_err_examples::run();
    custom_error_enum::run();
    validation_errors::run();
}
