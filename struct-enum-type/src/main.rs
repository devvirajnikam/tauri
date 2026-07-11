#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // takes &self so it can read the actual instance's name
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    let mut instance = Person::new(String::from("Jane"), 25);

    println!("{:?}", instance.name);

    let name = person.get_name();
    println!("{:?}", name);
    println!("{:?}", person.age);
    print!("Person NAME: {:?}", person.get_name());
    print!("Instance NAME: {:?}", instance.get_name());
    
    // Test set_name
    instance.set_name(String::from("Bob"));
    println!("Instance NAME after set: {:?}", instance.get_name());
}