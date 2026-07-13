// Scenario:
// A list needs to be split into two groups based on a condition.
//
// Thinking:
// partition() is useful when each item belongs in exactly one of two output
// collections.

pub fn run() {
    println!("\n27. partition()");

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (even, odd): (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| number % 2 == 0);

    println!("Even numbers: {:?}", even);
    println!("Odd numbers: {:?}", odd);
}
