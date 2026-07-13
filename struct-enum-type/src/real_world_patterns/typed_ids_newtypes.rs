// Scenario:
// A system has users, orders, and invoices. All IDs are numbers, but mixing a
// user ID with an order ID would be a real bug.
//
// Thinking:
// Wrap primitive values in small tuple structs, often called newtypes. The
// compiler then stops you from passing the wrong kind of ID to a function.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(u32);

#[derive(Debug)]
struct Order {
    id: OrderId,
    user_id: UserId,
}

fn load_orders_for_user(user_id: UserId) -> Vec<Order> {
    vec![Order {
        id: OrderId(101),
        user_id,
    }]
}

pub fn run() {
    println!("\n31. Typed IDs using newtype structs");

    let user_id = UserId(7);
    let orders = load_orders_for_user(user_id);

    println!("User id: {:?}", user_id);
    println!("Orders: {:?}", orders);
    for order in &orders {
        println!(
            "Order fields -> id: {:?}, user_id: {:?}",
            order.id, order.user_id
        );
    }
}
