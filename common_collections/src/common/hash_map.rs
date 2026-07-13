// Scenario:
// You need fast lookup from one value to another, such as item name to stock.
//
// Thinking:
// HashMap<K, V> is chosen when key lookup speed matters more than sorted order.

use std::collections::HashMap;

pub fn run() {
    println!("\n3. HashMap<K, V>");

    let mut stock_by_item = HashMap::new();
    stock_by_item.insert(String::from("Keyboard"), 10);
    stock_by_item.insert(String::from("Mouse"), 5);

    let keyboard = String::from("Keyboard");
    println!("Keyboard stock: {:?}", stock_by_item.get(&keyboard));

    let mouse_stock = stock_by_item.entry(String::from("Mouse")).or_insert(0);
    *mouse_stock += 2;

    for (item, stock) in &stock_by_item {
        println!("{} -> {}", item, stock);
    }
}
