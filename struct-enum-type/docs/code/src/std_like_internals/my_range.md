# my_range.rs

Source: `src\std_like_internals\my_range.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 10

```rust
struct MyRange {
```

Defines the struct ``MyRange``. A struct is chosen because this type needs named fields with clear meaning.

### Line 11

```rust
    start: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 12

```rust
    end: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 15

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 16

```rust
struct MyRangeInclusive {
```

Defines the struct ``MyRangeInclusive``. A struct is chosen because this type needs named fields with clear meaning.

### Line 17

```rust
    start: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 18

```rust
    end: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 21

```rust
impl MyRange {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 22

```rust
    fn contains(&self, value: i32) -> bool {
```

Defines the helper function ``contains``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 23

```rust
        value >= self.start && value < self.end
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 27

```rust
impl MyRangeInclusive {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 28

```rust
    fn contains(&self, value: i32) -> bool {
```

Defines the helper function ``contains``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 29

```rust
        value >= self.start && value <= self.end
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 33

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 34

```rust
    println!("\n49. Recreated Range structs");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 36

```rust
    let exclusive = MyRange { start: 1, end: 5 };
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 37

```rust
    let inclusive = MyRangeInclusive { start: 1, end: 5 };
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 39

```rust
    println!("{:?} contains 5: {}", exclusive, exclusive.contains(5));
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 40

```rust
    println!("{:?} contains 5: {}", inclusive, inclusive.contains(5));
```

Prints this value so running cargo run shows the behavior of the current lesson.
