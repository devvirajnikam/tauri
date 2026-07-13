// Scenario:
// Business systems often need an audit trail: user created, order paid, payment
// failed, stock changed, and so on.
//
// Thinking:
// Use an enum for domain events because every event has a clear name and
// event-specific data. A shared handler can match on all possible events.

#[derive(Debug)]
enum DomainEvent {
    UserCreated { user_id: u32, name: String },
    OrderPaid { order_id: u32, amount: u32 },
    StockAdjusted { item_name: String, change: i32 },
}

fn publish_event(event: DomainEvent) {
    match event {
        DomainEvent::UserCreated { user_id, name } => {
            println!("Audit: user {} created with name {}", user_id, name);
        }
        DomainEvent::OrderPaid { order_id, amount } => {
            println!("Audit: order {} paid amount {}", order_id, amount);
        }
        DomainEvent::StockAdjusted { item_name, change } => {
            println!("Audit: stock adjusted for {} by {}", item_name, change);
        }
    }
}

pub fn run() {
    println!("\n34. Domain event enums");

    let events = [
        DomainEvent::UserCreated {
            user_id: 1,
            name: String::from("Asha"),
        },
        DomainEvent::OrderPaid {
            order_id: 101,
            amount: 5000,
        },
        DomainEvent::StockAdjusted {
            item_name: String::from("Keyboard"),
            change: -2,
        },
    ];

    for event in events {
        publish_event(event);
    }
}
