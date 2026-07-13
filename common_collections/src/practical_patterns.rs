mod frequency_counter;
mod group_by_key;
mod indexed_lookup;
mod queue_processing;
mod remove_duplicates;
mod sort_and_search;

pub fn run() {
    println!("\nPractical collection patterns");

    frequency_counter::run();
    group_by_key::run();
    indexed_lookup::run();
    queue_processing::run();
    remove_duplicates::run();
    sort_and_search::run();
}
