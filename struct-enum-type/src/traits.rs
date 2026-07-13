trait Printable {
    fn print_summary(&self);
}

struct Person {
    name: String,
    age: u32,
}

enum Payment {
    Cash,
    Card { last_four_digits: String },
}

impl Printable for Person {
    fn print_summary(&self) {
        println!("Person summary: {} is {} years old", self.name, self.age);
    }
}

impl Printable for Payment {
    fn print_summary(&self) {
        match self {
            Payment::Cash => println!("Payment summary: paid by cash"),
            Payment::Card { last_four_digits } => {
                println!("Payment summary: paid by card ending {}", last_four_digits);
            }
        }
    }
}

fn print_anything_printable(value: &impl Printable) {
    value.print_summary();
}

pub fn run() {
    println!("\n8. Traits with structs and enums");

    let person = Person {
        name: String::from("Nina"),
        age: 28,
    };

    let cash_payment = Payment::Cash;
    let card_payment = Payment::Card {
        last_four_digits: String::from("4242"),
    };

    print_anything_printable(&person);
    print_anything_printable(&cash_payment);
    print_anything_printable(&card_payment);
}
