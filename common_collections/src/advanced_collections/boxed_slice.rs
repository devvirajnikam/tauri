// Scenario:
// You have a list that should not grow anymore after setup.
//
// Thinking:
// Box<[T]> is an owned fixed-size slice. It can be useful when you want owned
// heap data without Vec's extra capacity.

pub fn run() {
    println!("\n18. Box<[T]>");

    let numbers = vec![10, 20, 30];
    let fixed_numbers: Box<[i32]> = numbers.into_boxed_slice();

    println!("Fixed boxed slice: {:?}", fixed_numbers);
    println!("Length: {}", fixed_numbers.len());
}
