// Scenario:
// A program needs to look up values by key, such as item name to quantity.
//
// Thinking:
// HashMap is a struct in real Rust. A real HashMap uses hashing and buckets.
// This tiny version uses Vec<(K, V)> pairs so the key-value idea is visible.

#[derive(Debug)]
struct MyHashMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K: PartialEq, V> MyHashMap<K, V> {
    fn new() -> MyHashMap<K, V> {
        MyHashMap {
            entries: Vec::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        for (existing_key, existing_value) in &mut self.entries {
            if *existing_key == key {
                *existing_value = value;
                return;
            }
        }

        self.entries.push((key, value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        for (existing_key, existing_value) in &self.entries {
            if existing_key == key {
                return Some(existing_value);
            }
        }

        None
    }
}

pub fn run() {
    println!("\n53. Recreated HashMap-like struct");

    let mut quantities = MyHashMap::new();
    quantities.insert(String::from("Keyboard"), 10);
    quantities.insert(String::from("Mouse"), 5);
    quantities.insert(String::from("Keyboard"), 12);

    println!("{:?}", quantities);
    println!(
        "Keyboard quantity: {:?}",
        quantities.get(&String::from("Keyboard"))
    );
}
