// Scenario:
// The same wrapper shape should work for different inner data types.
//
// Thinking:
// A generic struct stores a value whose concrete type is chosen when the struct
// is used.

#[derive(Debug)]
struct BoxedValue<T> {
    value: T,
}

pub fn run() {
    println!("\n2. Generic struct");

    let number_box = BoxedValue { value: 42 };
    let text_box = BoxedValue {
        value: String::from("hello"),
    };

    println!("Number box: {:?}", number_box);
    println!("Number value: {}", number_box.value);
    println!("Text box: {:?}", text_box);
    println!("Text value: {}", text_box.value);
}
