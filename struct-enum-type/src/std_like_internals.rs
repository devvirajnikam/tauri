// Scenario:
// Rust gives us very common standard library types like Option, Result, Vec,
// String, Box, Range, Ordering, and ControlFlow.
//
// Thinking:
// Recreating small versions of these types makes their internal design easier to
// understand. These examples are simplified learning models, not replacements
// for the real standard library types.

mod my_box;
mod my_cell;
mod my_control_flow;
mod my_hash_map;
mod my_ip_addr;
mod my_option;
mod my_ordering;
mod my_poll;
mod my_range;
mod my_result;
mod my_string;
mod my_vec;

pub fn run() {
    println!("\nStd-like internals recreated for learning");

    my_option::run();
    my_result::run();
    my_ordering::run();
    my_control_flow::run();
    my_poll::run();
    my_range::run();
    my_box::run();
    my_vec::run();
    my_string::run();
    my_hash_map::run();
    my_ip_addr::run();
    my_cell::run();
}
