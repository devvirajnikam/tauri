// Scenario:
// A generic function needs to call trait methods on its input.
//
// Thinking:
// Trait bounds restrict generic types to values that support required behavior.

trait Printable {
    fn print(&self);
}

struct Invoice {
    number: String,
}

impl Printable for Invoice {
    fn print(&self) {
        println!("Invoice number: {}", self.number);
    }
}

fn print_anything<T: Printable>(value: &T) {
    value.print();
}

pub fn run() {
    println!("\n3. Trait bounds");

    let invoice = Invoice {
        number: String::from("INV-001"),
    };

    print_anything(&invoice);
}
