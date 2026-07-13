// Scenario:
// You need owned, growable text, such as user input or a generated message.
//
// Thinking:
// String is a UTF-8 collection of bytes with text-specific methods. Use String
// when the program owns and may change the text.

pub fn run() {
    println!("\n2. String");

    let mut message = String::from("Hello");
    message.push(',');
    message.push_str(" Rust");

    println!("Message: {}", message);
    println!("Bytes length: {}", message.len());

    for character in message.chars() {
        println!("Character: {}", character);
    }

    let words: Vec<&str> = message.split_whitespace().collect();
    println!("Words: {:?}", words);
}
