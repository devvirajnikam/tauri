// Scenario:
// Some operations should only be allowed after an object reaches a certain
// compile-time state, such as draft invoice vs approved invoice.
//
// Thinking:
// PhantomData lets a struct carry type-level state without storing extra runtime
// data. Methods can then exist only for the correct state.

use std::marker::PhantomData;

#[derive(Debug)]
struct Draft;

#[derive(Debug)]
struct Approved;

#[derive(Debug)]
struct Invoice<State> {
    id: u32,
    total: u32,
    state: PhantomData<State>,
}

impl Invoice<Draft> {
    fn new(id: u32, total: u32) -> Invoice<Draft> {
        Invoice {
            id,
            total,
            state: PhantomData,
        }
    }

    fn approve(self) -> Invoice<Approved> {
        Invoice {
            id: self.id,
            total: self.total,
            state: PhantomData,
        }
    }
}

impl Invoice<Approved> {
    fn send_to_customer(&self) {
        println!("Sending approved invoice {} for {}", self.id, self.total);
    }
}

pub fn run() {
    println!("\n39. PhantomData state types");

    let draft_invoice = Invoice::<Draft>::new(501, 9000);
    let approved_invoice = draft_invoice.approve();

    println!("{:?}", approved_invoice);
    approved_invoice.send_to_customer();
}
