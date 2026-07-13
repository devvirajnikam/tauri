# if_while_let.rs

Source: `src\advanced_topics\if_while_let.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 2

```rust
    println!("\n11. if let and while let");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 4

```rust
    let maybe_name = Some(String::from("John"));
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 6

```rust
    if let Some(name) = maybe_name {
```

Uses if let to handle one pattern directly without writing a full match for every possible case.

### Line 7

```rust
        println!("if let found name: {}", name);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 10

```rust
    let mut numbers = vec![1, 2, 3];
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 12

```rust
    while let Some(number) = numbers.pop() {
```

Uses while let to keep looping as long as the expression keeps producing the matching pattern.

### Line 13

```rust
        println!("while let popped: {}", number);
```

Prints this value so running cargo run shows the behavior of the current lesson.
