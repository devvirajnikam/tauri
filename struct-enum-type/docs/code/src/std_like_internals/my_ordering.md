# my_ordering.rs

Source: `src\std_like_internals\my_ordering.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 8

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 9

```rust
enum MyOrdering {
```

Defines the enum ``MyOrdering``. An enum is chosen when the value must be exactly one case from a known set.

### Line 10

```rust
    Less,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 11

```rust
    Equal,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    Greater,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
fn compare_numbers(left: i32, right: i32) -> MyOrdering {
```

Defines the helper function ``compare_numbers``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 16

```rust
    if left < right {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 17

```rust
        MyOrdering::Less
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 18

```rust
    } else if left > right {
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 19

```rust
        MyOrdering::Greater
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 20

```rust
    } else {
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 21

```rust
        MyOrdering::Equal
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 25

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 26

```rust
    println!("\n46. Recreated Ordering enum");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 28

```rust
    println!("3 vs 5: {:?}", compare_numbers(3, 5));
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 29

```rust
    println!("5 vs 5: {:?}", compare_numbers(5, 5));
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 30

```rust
    println!("9 vs 5: {:?}", compare_numbers(9, 5));
```

Prints this value so running cargo run shows the behavior of the current lesson.
