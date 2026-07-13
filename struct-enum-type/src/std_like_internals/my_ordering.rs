// Scenario:
// Sorting and comparisons need to describe whether one value is less than,
// equal to, or greater than another value.
//
// Thinking:
// Ordering is an enum because comparison has exactly three possible outcomes.

#[derive(Debug)]
enum MyOrdering {
    Less,
    Equal,
    Greater,
}

fn compare_numbers(left: i32, right: i32) -> MyOrdering {
    if left < right {
        MyOrdering::Less
    } else if left > right {
        MyOrdering::Greater
    } else {
        MyOrdering::Equal
    }
}

pub fn run() {
    println!("\n46. Recreated Ordering enum");

    println!("3 vs 5: {:?}", compare_numbers(3, 5));
    println!("5 vs 5: {:?}", compare_numbers(5, 5));
    println!("9 vs 5: {:?}", compare_numbers(9, 5));
}
