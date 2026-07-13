# lifetimes.rs

Source: `src\advanced_topics\lifetimes.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct NameView<'a> {
```

Defines the generic struct ``NameView``. Generics are chosen so the same wrapper or container can work with different inner data types.

### Line 3

```rust
    first_name: &'a str,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 4

```rust
    last_name: &'a str,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 7

```rust
impl<'a> NameView<'a> {
```

Starts a generic implementation block. The type parameters make these methods available for many concrete versions of the type.

### Line 8

```rust
    fn full_name(&self) -> String {
```

Defines the helper function ``full_name``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 9

```rust
        format!("{} {}", self.first_name, self.last_name)
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 14

```rust
    println!("\n16. Lifetimes inside structs");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 16

```rust
    let first_name = String::from("John");
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 17

```rust
    let last_name = String::from("Doe");
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 19

```rust
    let name_view = NameView {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 20

```rust
        first_name: &first_name,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 21

```rust
        last_name: &last_name,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 24

```rust
    println!("{:?}", name_view);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 25

```rust
    println!("Full name: {}", name_view.full_name());
```

Prints this value so running cargo run shows the behavior of the current lesson.
