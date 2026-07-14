mod a_define_and_impl;
mod b_default_methods;
mod c_trait_bounds;
mod d_trait_parameters;

pub fn run() {
    println!("\nTrait basics");

    a_define_and_impl::run();
    b_default_methods::run();
    c_trait_bounds::run();
    d_trait_parameters::run();
}
