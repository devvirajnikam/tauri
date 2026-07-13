# option_result_helpers.rs

Source: `src\advanced_topics\option_result_helpers.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
fn parse_quantity(input: &str) -> Result<u32, String> {
```

Defines the helper function ``parse_quantity``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 2

```rust
    input
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 3

```rust
        .parse::<u32>()
```

Continues chained access from the previous line.

### Line 4

```rust
        .map_err(|error| format!("invalid quantity: {}", error))
```

Continues a method chain, where this method call configures or transforms the value from the previous line.

### Line 7

```rust
fn double_optional_number(number: Option<u32>) -> Option<u32> {
```

Defines the helper function ``double_optional_number``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 8

```rust
    number.map(|value| value * 2)
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 11

```rust
fn calculate_total(quantity_text: &str, price: u32) -> Result<u32, String> {
```

Defines the helper function ``calculate_total``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 12

```rust
    let quantity = parse_quantity(quantity_text)?;
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 13

```rust
    Ok(quantity * price)
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 16

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 17

```rust
    println!("\n20. Option and Result helpers");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 19

```rust
    let doubled = double_optional_number(Some(10));
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 20

```rust
    let fallback = double_optional_number(None).unwrap_or(0);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 22

```rust
    println!("Option map result: {:?}", doubled);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 23

```rust
    println!("Option unwrap_or fallback: {}", fallback);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 25

```rust
    match calculate_total("4", 25) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 26

```rust
        Ok(total) => println!("Total is {}", total),
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 27

```rust
        Err(error) => println!("Error: {}", error),
```

Creates an error result. Err carries the reason the operation failed.

### Line 30

```rust
    match calculate_total("abc", 25) {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 31

```rust
        Ok(total) => println!("Total is {}", total),
```

Creates a success result. Ok carries the value produced by a successful operation.

### Line 32

```rust
        Err(error) => println!("Error after map_err and ?: {}", error),
```

Creates an error result. Err carries the reason the operation failed.
