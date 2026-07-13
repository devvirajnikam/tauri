# enum_with_data.rs

Source: `src\enum_with_data.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
enum Message {
```

Defines the enum ``Message``. An enum is chosen when the value must be exactly one case from a known set.

### Line 3

```rust
    Text(String),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 4

```rust
    Number(i32),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 5

```rust
    Move { x: i32, y: i32 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 6

```rust
    Quit,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 9

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 10

```rust
    println!("\n4. Enum with data");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 12

```rust
    let messages = [
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 13

```rust
        Message::Text(String::from("hello")),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
        Message::Number(42),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
        Message::Move { x: 10, y: 20 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
        Message::Quit,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 19

```rust
    for message in messages {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 20

```rust
        println!("{:?}", message);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 22

```rust
        match message {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 23

```rust
            Message::Text(text) => println!("Text message: {}", text),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 24

```rust
            Message::Number(number) => println!("Number message: {}", number),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 25

```rust
            Message::Move { x, y } => println!("Move to x={}, y={}", x, y),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 26

```rust
            Message::Quit => println!("Quit message"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.
