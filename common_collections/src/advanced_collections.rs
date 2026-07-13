mod arrays_capacity;
mod boxed_slice;
mod cell_refcell;
mod collect_examples;
mod cow_examples;
mod from_iterator_extend;
mod iterator_adapters;
mod nested_collections;
mod option_result_containers;
mod partition_examples;
mod rc_arc_weak;
mod str_slices;
mod third_party_overview;
mod vec_mutation_methods;

pub fn run() {
    println!("\nAdvanced collection-like types and patterns");

    str_slices::run();
    boxed_slice::run();
    cow_examples::run();
    rc_arc_weak::run();
    cell_refcell::run();
    option_result_containers::run();
    iterator_adapters::run();
    from_iterator_extend::run();
    collect_examples::run();
    vec_mutation_methods::run();
    partition_examples::run();
    nested_collections::run();
    arrays_capacity::run();
    third_party_overview::run();
}
