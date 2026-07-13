mod arrays_and_slices;
mod binary_heap;
mod btree_map;
mod btree_set;
mod hash_map;
mod hash_set;
mod linked_list;
mod string_collection;
mod vec_collection;
mod vec_deque;

pub fn run() {
    println!("\nCommon Rust collections");

    vec_collection::run();
    string_collection::run();
    hash_map::run();
    hash_set::run();
    vec_deque::run();
    btree_map::run();
    btree_set::run();
    binary_heap::run();
    linked_list::run();
    arrays_and_slices::run();
}
