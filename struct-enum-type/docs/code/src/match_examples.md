# match_examples.rs

Source: `src\match_examples.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
enum OrderStatus {
```

Defines the enum ``OrderStatus``. An enum is chosen when the value must be exactly one case from a known set.

### Line 3

```rust
    Pending,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 4

```rust
    Paid,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 5

```rust
    Shipped,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 6

```rust
    Cancelled,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 9

```rust
fn status_message(status: &OrderStatus) -> &'static str {
```

Defines the helper function ``status_message``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 10

```rust
    match status {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 11

```rust
        OrderStatus::Pending => "Order is waiting for payment",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 12

```rust
        OrderStatus::Paid => "Order payment is complete",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 13

```rust
        OrderStatus::Shipped => "Order is on the way",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 14

```rust
        OrderStatus::Cancelled => "Order was cancelled",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 18

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 19

```rust
    println!("\n5. match");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 21

```rust
    let statuses = [
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 22

```rust
        OrderStatus::Pending,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 23

```rust
        OrderStatus::Paid,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 24

```rust
        OrderStatus::Shipped,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 25

```rust
        OrderStatus::Cancelled,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 28

```rust
    for status in &statuses {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 29

```rust
        println!("{:?}: {}", status, status_message(&status));
```

Prints this value so running cargo run shows the behavior of the current lesson.
