mod a_trait_objects;
mod b_associated_types;
mod c_supertraits;
mod d_where_clause_bounds;
mod e_operator_traits;
mod f_derived_traits;

pub fn run() {
    println!("\nAdvanced trait topics");

    a_trait_objects::run();
    b_associated_types::run();
    c_supertraits::run();
    d_where_clause_bounds::run();
    e_operator_traits::run();
    f_derived_traits::run();
}
