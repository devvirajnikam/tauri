// Scenario:
// An inventory system needs to store many item names and their quantities.
//
// Thinking:
// A struct is the owner of the inventory behavior, while HashMap is the best
// field type when each item name should quickly point to one current quantity.

use std::collections::HashMap;

#[derive(Debug)]
struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }

    fn add_stock(&mut self, item_name: String, quantity: u32) {
        let current_quantity = self.items.entry(item_name).or_insert(0);
        *current_quantity += quantity;
    }

    fn quantity(&self, item_name: &str) -> u32 {
        self.items.get(item_name).copied().unwrap_or(0)
    }
}

pub fn run() {
    println!("\n21. HashMap fields inside structs");

    let mut inventory = Inventory::new();
    inventory.add_stock(String::from("Keyboard"), 10);
    inventory.add_stock(String::from("Keyboard"), 5);
    inventory.add_stock(String::from("Mouse"), 8);

    println!("{:?}", inventory);
    println!("Keyboard quantity: {}", inventory.quantity("Keyboard"));
}
