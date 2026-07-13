// Scenario:
// You receive a list but later need fast lookup by ID.
//
// Thinking:
// Convert Vec<Record> into HashMap<Id, Record> when repeated lookup by ID is more
// important than preserving the original list shape.

use std::collections::HashMap;

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
}

pub fn run() {
    println!("\n13. Indexed lookup");

    let products = vec![
        Product {
            id: 1,
            name: String::from("Keyboard"),
        },
        Product {
            id: 2,
            name: String::from("Mouse"),
        },
    ];

    let products_by_id: HashMap<u32, Product> = products
        .into_iter()
        .map(|product| (product.id, product))
        .collect();

    if let Some(product) = products_by_id.get(&2) {
        println!("Found product: {:?}", product);
        println!("Product name: {}", product.name);
    }
}
