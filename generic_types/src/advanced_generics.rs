mod associated_types;
mod const_generics;
mod generic_lifetimes;
mod generic_methods;
mod marker_phantom_data;
mod multiple_type_parameters;

pub fn run() {
    println!("\nAdvanced generic topics");

    multiple_type_parameters::run();
    generic_methods::run();
    generic_lifetimes::run();
    associated_types::run();
    const_generics::run();
    marker_phantom_data::run();
}
