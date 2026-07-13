// Scenario:
// An order moves through a strict workflow: Draft, Submitted, Paid, or Cancelled.
//
// Thinking:
// Use an enum as a state machine when a value can only be in one valid state at a
// time. Variants can store state-specific data like transaction_id or reason.

#[derive(Debug)]
struct Order {
    id: u32,
    state: OrderState,
}

#[derive(Debug)]
enum OrderState {
    Draft,
    Submitted,
    Paid { transaction_id: String },
    Cancelled { reason: String },
}

impl Order {
    fn submit(&mut self) {
        if matches!(self.state, OrderState::Draft) {
            self.state = OrderState::Submitted;
        }
    }

    fn pay(&mut self, transaction_id: String) {
        if matches!(self.state, OrderState::Submitted) {
            self.state = OrderState::Paid { transaction_id };
        }
    }

    fn cancel(&mut self, reason: String) {
        if !matches!(self.state, OrderState::Paid { .. }) {
            self.state = OrderState::Cancelled { reason };
        }
    }
}

pub fn run() {
    println!("\n29. Enums as state machines");

    let mut order = Order {
        id: 101,
        state: OrderState::Draft,
    };

    println!("Initial order: {:?}", order);
    println!("Order id: {}", order.id);

    order.submit();
    println!("After submit: {:?}", order);

    order.pay(String::from("TXN-123"));
    println!("After pay: {:?}", order);

    match &order.state {
        OrderState::Paid { transaction_id } => {
            println!("Paid transaction id: {}", transaction_id);
        }
        OrderState::Cancelled { reason } => println!("Cancelled reason: {}", reason),
        OrderState::Draft => println!("Order is still draft"),
        OrderState::Submitted => println!("Order is submitted"),
    }

    let mut cancelled_order = Order {
        id: 102,
        state: OrderState::Draft,
    };
    cancelled_order.cancel(String::from("Customer changed mind"));
    println!("Cancelled order: {:?}", cancelled_order);
}
