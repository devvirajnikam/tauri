pub fn run() {
    println!("\n11. if let and while let");

    let maybe_name = Some(String::from("John"));

    if let Some(name) = maybe_name {
        println!("if let found name: {}", name);
    }

    let mut numbers = vec![1, 2, 3];

    while let Some(number) = numbers.pop() {
        println!("while let popped: {}", number);
    }
}
