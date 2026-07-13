// Scenario:
// A type needs to store two different kinds of values.
//
// Thinking:
// Multiple type parameters let each field choose its own type.

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

pub fn run() {
    println!("\n11. Multiple type parameters");

    let setting = KeyValue {
        key: "theme",
        value: String::from("dark"),
    };

    println!("Setting: {:?}", setting);
    println!(
        "Setting fields -> key: {}, value: {}",
        setting.key, setting.value
    );
}
