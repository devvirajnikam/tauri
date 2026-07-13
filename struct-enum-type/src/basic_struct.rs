#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

pub fn run() {
    println!("\n1. Basic struct");

    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("Full person: {:?}", person);
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}
