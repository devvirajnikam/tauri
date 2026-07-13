// Scenario:
// Multiple entities use IDs, but IDs should not be mixed accidentally.
//
// Thinking:
// A generic newtype can tag an ID with the entity type it belongs to.

use std::marker::PhantomData;

#[derive(Debug)]
struct Id<T> {
    value: u32,
    marker: PhantomData<T>,
}

#[derive(Debug)]
struct User;

#[derive(Debug)]
struct Order;

impl<T> Id<T> {
    fn new(value: u32) -> Id<T> {
        Id {
            value,
            marker: PhantomData,
        }
    }
}

pub fn run() {
    println!("\n10. Generic typed IDs");

    let user_id: Id<User> = Id::new(1);
    let order_id: Id<Order> = Id::new(1);

    println!("User ID: {:?}", user_id);
    println!("User ID raw value: {}", user_id.value);
    println!("Order ID: {:?}", order_id);
    println!("Order ID raw value: {}", order_id.value);
}
