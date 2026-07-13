// Scenario:
// Text needs an owned, growable container.
//
// Thinking:
// String is a struct in real Rust. Internally it is backed by bytes that must be
// valid UTF-8. This simplified version stores characters to make the idea easy.

#[derive(Debug)]
struct MyString {
    characters: Vec<char>,
}

impl MyString {
    fn new() -> MyString {
        MyString {
            characters: Vec::new(),
        }
    }

    fn push(&mut self, character: char) {
        self.characters.push(character);
    }

    fn len(&self) -> usize {
        self.characters.len()
    }

    fn as_real_string(&self) -> String {
        self.characters.iter().collect()
    }
}

pub fn run() {
    println!("\n52. Recreated String-like struct");

    let mut text = MyString::new();
    text.push('R');
    text.push('u');
    text.push('s');
    text.push('t');

    println!("{:?}", text);
    println!("Length: {}", text.len());
    println!("As String: {}", text.as_real_string());
}
