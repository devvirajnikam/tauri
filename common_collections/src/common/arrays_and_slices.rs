// Scenario:
// Sometimes the number of items is fixed, or a function should only borrow part
// of an existing collection.
//
// Thinking:
// Arrays have fixed length. Slices borrow a view into an array or Vec without
// taking ownership.

fn print_slice(values: &[i32]) {
    for value in values {
        println!("Slice value: {}", value);
    }
}

pub fn run() {
    println!("\n10. Arrays and slices");

    let fixed_numbers = [10, 20, 30, 40];
    println!("Array: {:?}", fixed_numbers);

    let middle = &fixed_numbers[1..3];
    print_slice(middle);
}
