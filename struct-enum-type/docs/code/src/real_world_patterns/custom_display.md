# custom_display.rs

Source: `src\real_world_patterns\custom_display.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
use std::fmt;
```

Imports a type, trait, or module path so this file can use a shorter name instead of repeating the full path.

### Line 11

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 12

```rust
enum PaymentStatus {
```

Defines the enum ``PaymentStatus``. An enum is chosen when the value must be exactly one case from a known set.

### Line 13

```rust
    Pending,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
    Completed,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
    Failed,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 18

```rust
impl fmt::Display for PaymentStatus {
```

Implements ``fmt::Display`` for ``PaymentStatus``. This is chosen when the type should support an existing behavior contract.

### Line 19

```rust
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
```

Defines the helper function ``fmt``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 20

```rust
        match self {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 21

```rust
            PaymentStatus::Pending => write!(formatter, "Pending"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 22

```rust
            PaymentStatus::Completed => write!(formatter, "Completed"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 23

```rust
            PaymentStatus::Failed => write!(formatter, "Failed"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 28

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 29

```rust
    println!("\n25. Custom Display");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 31

```rust
    let statuses = [
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 32

```rust
        PaymentStatus::Pending,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 33

```rust
        PaymentStatus::Completed,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 34

```rust
        PaymentStatus::Failed,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 37

```rust
    for status in statuses {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 38

```rust
        println!("Debug: {:?}, Display: {}", status, status);
```

Prints this value so running cargo run shows the behavior of the current lesson.
