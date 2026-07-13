#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
    Move { x: i32, y: i32 },
    Quit,
}

pub fn run() {
    println!("\n4. Enum with data");

    let messages = [
        Message::Text(String::from("hello")),
        Message::Number(42),
        Message::Move { x: 10, y: 20 },
        Message::Quit,
    ];

    for message in messages {
        println!("{:?}", message);

        match message {
            Message::Text(text) => println!("Text message: {}", text),
            Message::Number(number) => println!("Number message: {}", number),
            Message::Move { x, y } => println!("Move to x={}, y={}", x, y),
            Message::Quit => println!("Quit message"),
        }
    }
}
