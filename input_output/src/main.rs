mod command_loop;
mod formatted_output;
mod parse_input;
mod print_and_flush;
mod result_handling;
mod reusable_reader;
mod simple_read;
mod stderr_output;
mod write_trait_output;

fn main() {
    simple_read::run();
    result_handling::run();
    parse_input::run();
    reusable_reader::run();
    command_loop::run();
    print_and_flush::run();
    formatted_output::run();
    stderr_output::run();
    write_trait_output::run();
}
