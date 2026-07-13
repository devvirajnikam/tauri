# option_result.rs

Source: `src\option_result.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
fn find_user_name(id: u32) -> Option<String> {
```

Defines the helper function ``find_user_name``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 2

```rust
    if id == 1 {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 3

```rust
        Some(String::from("John"))
```

Creates an optional value that is present. Some wraps the actual value.

### Line 4

```rust
    } else {
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 5

```rust
        None
```

Represents an optional value that is missing, avoiding null.

### Line 9

```rust
fn divide(left: i32, right: i32) -> Result<i32, String> {
```

Defines the helper function ``divide``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 10

```rust
    if right == 0 {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 11

```rust
        Err(String::from("cannot divide by zero"))
```

Creates an error result. Err carries the reason the operation failed.

### Line 12

```rust
    } else {
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 13

```rust
        Ok(left / right)
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 17

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 18

```rust
    println!("\n6. Option and Result");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 20

```rust
    match find_user_name(1) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 21

```rust
        Some(name) => println!("Found user: {}", name),
```

Creates an optional value that is present. Some wraps the actual value.

### Line 22

```rust
        None => println!("User not found"),
```

Represents an optional value that is missing, avoiding null.

### Line 25

```rust
    match find_user_name(99) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 26

```rust
        Some(name) => println!("Found user: {}", name),
```

Creates an optional value that is present. Some wraps the actual value.

### Line 27

```rust
        None => println!("User not found"),
```

Represents an optional value that is missing, avoiding null.

### Line 30

```rust
    match divide(10, 2) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 31

```rust
        Ok(answer) => println!("10 / 2 = {}", answer),
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 32

```rust
        Err(error) => println!("Error: {}", error),
```

Creates an error result. Err carries the reason the operation failed.

### Line 35

```rust
    match divide(10, 0) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 36

```rust
        Ok(answer) => println!("10 / 0 = {}", answer),
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 37

```rust
        Err(error) => println!("Error: {}", error),
```

Creates an error result. Err carries the reason the operation failed.
