// Scenario:
// Some loops or tree walks should continue normally, but sometimes they should
// stop early and return a value.
//
// Thinking:
// ControlFlow is an enum because a process either Continues or Breaks with a
// reason/value.

#[derive(Debug)]
enum MyControlFlow<B, C> {
    Break(B),
    Continue(C),
}

fn find_first_even(numbers: &[i32]) -> MyControlFlow<i32, ()> {
    for number in numbers {
        if number % 2 == 0 {
            return MyControlFlow::Break(*number);
        }
    }

    MyControlFlow::Continue(())
}

pub fn run() {
    println!("\n47. Recreated ControlFlow enum");

    let result = find_first_even(&[1, 3, 5, 8, 9]);
    println!("Search result: {:?}", result);
}
