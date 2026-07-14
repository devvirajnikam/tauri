// Scenario:
// A collection needs to store different concrete types that share one behavior.
//
// Thinking:
// Box<dyn Trait> creates a trait object for dynamic dispatch.

trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

struct TextInput {
    placeholder: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

impl Draw for TextInput {
    fn draw(&self) {
        println!("Drawing input: {}", self.placeholder);
    }
}

pub fn run() {
    println!("\n10. Trait objects");

    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            label: String::from("Save"),
        }),
        Box::new(TextInput {
            placeholder: String::from("Enter name"),
        }),
    ];

    for component in components {
        component.draw();
    }
}
