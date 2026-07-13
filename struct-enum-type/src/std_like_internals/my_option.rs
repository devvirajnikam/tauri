// Scenario:
// Sometimes a value may exist, and sometimes it may be missing.
//
// Thinking:
// Option is an enum because there are exactly two states: Some(value) or None.
// This removes the need for null values.

#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn is_some(&self) -> bool {
        matches!(self, MyOption::Some(_))
    }

    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback,
        }
    }

    fn map<U>(self, convert: fn(T) -> U) -> MyOption<U> {
        match self {
            MyOption::Some(value) => MyOption::Some(convert(value)),
            MyOption::None => MyOption::None,
        }
    }
}

pub fn run() {
    println!("\n44. Recreated Option enum");

    let name = MyOption::Some(String::from("Asha"));
    let missing_name: MyOption<String> = MyOption::None;

    println!("Has name: {}", name.is_some());
    println!(
        "Missing fallback: {}",
        missing_name.unwrap_or(String::from("Guest"))
    );

    let doubled = MyOption::Some(10).map(|number| number * 2);
    println!("Mapped option: {:?}", doubled);
}
