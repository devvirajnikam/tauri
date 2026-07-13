# my_control_flow.rs

Source: `src\std_like_internals\my_control_flow.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 10

```rust
enum MyControlFlow<B, C> {
```

Defines the generic enum ``MyControlFlow``. A generic enum is chosen when each variant may carry values whose concrete types are decided by the caller.

### Line 11

```rust
    Break(B),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    Continue(C),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
fn find_first_even(numbers: &[i32]) -> MyControlFlow<i32, ()> {
```

Defines the helper function ``find_first_even``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 16

```rust
    for number in numbers {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 17

```rust
        if number % 2 == 0 {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 18

```rust
            return MyControlFlow::Break(*number);
```

Returns early because the function already knows the final answer or error at this point.

### Line 22

```rust
    MyControlFlow::Continue(())
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 25

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 26

```rust
    println!("\n47. Recreated ControlFlow enum");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 28

```rust
    let result = find_first_even(&[1, 3, 5, 8, 9]);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 29

```rust
    println!("Search result: {:?}", result);
```

Prints this value so running cargo run shows the behavior of the current lesson.
