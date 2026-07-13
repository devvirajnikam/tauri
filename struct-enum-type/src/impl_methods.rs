#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}

pub fn run() {
    println!("\n2. impl methods");

    let mut person = Person::new(String::from("Jane"), 25);

    println!("Before update: {:?}", person);
    println!("Read name using &self: {}", person.get_name());

    person.set_name(String::from("Bob"));
    person.birthday();

    println!("After update using &mut self: {:?}", person);
}
