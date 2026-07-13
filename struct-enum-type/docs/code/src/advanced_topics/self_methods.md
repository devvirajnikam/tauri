# self_methods.rs

Source: `src\advanced_topics\self_methods.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct DraftPost {
```

Defines the struct ``DraftPost``. A struct is chosen because this type needs named fields with clear meaning.

### Line 3

```rust
    title: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 6

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 7

```rust
struct PublishedPost {
```

Defines the struct ``PublishedPost``. A struct is chosen because this type needs named fields with clear meaning.

### Line 8

```rust
    title: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 11

```rust
impl DraftPost {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 12

```rust
    fn new(title: String) -> DraftPost {
```

Defines the helper function ``new``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 13

```rust
        DraftPost { title }
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
    fn publish(self) -> PublishedPost {
```

Defines the helper function ``publish``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 17

```rust
        PublishedPost { title: self.title }
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 21

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 22

```rust
    println!("\n12. Methods that consume self");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 24

```rust
    let draft = DraftPost::new(String::from("Rust structs and enums"));
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 25

```rust
    let published = draft.publish();
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 27

```rust
    println!("Published post: {:?}", published);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 28

```rust
    println!("Published title: {}", published.title);
```

Prints this value so running cargo run shows the behavior of the current lesson.
