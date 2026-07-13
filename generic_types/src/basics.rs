mod generic_enum;
mod generic_function;
mod generic_impl;
mod generic_struct;

pub fn run() {
    println!("\nGeneric type basics");

    generic_function::run();
    generic_struct::run();
    generic_enum::run();
    generic_impl::run();
}
