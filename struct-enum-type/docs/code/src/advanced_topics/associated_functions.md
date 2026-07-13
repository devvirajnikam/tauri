# associated_functions.rs

Source: `src\advanced_topics\associated_functions.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct Rectangle {
```

Defines the struct ``Rectangle``. A struct is chosen because this type needs named fields with clear meaning.

### Line 3

```rust
    width: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 4

```rust
    height: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 7

```rust
impl Rectangle {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 8

```rust
    fn new(width: u32, height: u32) -> Rectangle {
```

Defines the helper function ``new``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 9

```rust
        Rectangle { width, height }
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    fn square(size: u32) -> Rectangle {
```

Defines the helper function ``square``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 13

```rust
        Rectangle {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
            width: size,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 15

```rust
            height: size,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 19

```rust
    fn area(&self) -> u32 {
```

Defines the helper function ``area``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 20

```rust
        self.width * self.height
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 24

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 25

```rust
    println!("\n13. Associated functions vs methods");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 27

```rust
    let rectangle = Rectangle::new(10, 20);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 28

```rust
    let square = Rectangle::square(5);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 30

```rust
    println!("Rectangle: {:?}, area={}", rectangle, rectangle.area());
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 31

```rust
    println!("Square: {:?}, area={}", square, square.area());
```

Prints this value so running cargo run shows the behavior of the current lesson.
