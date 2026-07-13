# destructuring.rs

Source: `src\advanced_topics\destructuring.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct Point {
```

Defines the struct ``Point``. A struct is chosen because this type needs named fields with clear meaning.

### Line 3

```rust
    x: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 4

```rust
    y: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 7

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 8

```rust
enum Shape {
```

Defines the enum ``Shape``. An enum is chosen when the value must be exactly one case from a known set.

### Line 9

```rust
    Circle { radius: u32 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 10

```rust
    Rectangle { width: u32, height: u32 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 14

```rust
    println!("\n10. Destructuring structs and enums");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 16

```rust
    let point = Point { x: 5, y: 9 };
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 17

```rust
    let Point { x, y } = point;
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 18

```rust
    println!("Point parts -> x={}, y={}", x, y);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 20

```rust
    let shapes = [
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 21

```rust
        Shape::Circle { radius: 10 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 22

```rust
        Shape::Rectangle {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 23

```rust
            width: 20,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 24

```rust
            height: 30,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 28

```rust
    for shape in shapes {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 29

```rust
        match shape {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 30

```rust
            Shape::Circle { radius } => println!("Circle radius: {}", radius),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 31

```rust
            Shape::Rectangle { width, height } => {
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 32

```rust
                println!("Rectangle size: {} x {}", width, height);
```

Prints this value so running cargo run shows the behavior of the current lesson.
