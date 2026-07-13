// Scenario:
// An operation may succeed with a value or fail with an error.
//
// Thinking:
// Result is an enum because success and failure are different states with
// different data. Ok carries the success value. Err carries the error.

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MyResult<T, E> {
    fn is_ok(&self) -> bool {
        matches!(self, MyResult::Ok(_))
    }

    fn map<U>(self, convert: fn(T) -> U) -> MyResult<U, E> {
        match self {
            MyResult::Ok(value) => MyResult::Ok(convert(value)),
            MyResult::Err(error) => MyResult::Err(error),
        }
    }

    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => fallback,
        }
    }
}

fn divide(left: i32, right: i32) -> MyResult<i32, String> {
    if right == 0 {
        MyResult::Err(String::from("cannot divide by zero"))
    } else {
        MyResult::Ok(left / right)
    }
}

pub fn run() {
    println!("\n45. Recreated Result enum");

    let success = divide(10, 2);
    let failure = divide(10, 0);
    let fallback_failure = divide(10, 0);

    println!("Success: {:?}, is_ok={}", success, success.is_ok());
    println!("Failure: {:?}", failure);
    println!("Failure fallback: {}", fallback_failure.unwrap_or(0));

    let mapped = divide(20, 2).map(|number| number + 1);
    println!("Mapped result: {:?}", mapped);
}
