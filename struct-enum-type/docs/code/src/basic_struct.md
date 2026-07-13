# basic_struct.rs

Source: `src\basic_struct.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct Person {
```

Defines the struct ``Person``. A struct is chosen because this type needs named fields with clear meaning.

### Line 3

```rust
    name: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 4

```rust
    age: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 7

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 8

```rust
    println!("\n1. Basic struct");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 10

```rust
    let person = Person {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 11

```rust
        name: String::from("John"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
        age: 30,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
    println!("Full person: {:?}", person);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 16

```rust
    println!("Name: {}", person.name);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 17

```rust
    println!("Age: {}", person.age);
```

Prints this value so running cargo run shows the behavior of the current lesson.
