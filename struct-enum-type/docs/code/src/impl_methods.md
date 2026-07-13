# impl_methods.rs

Source: `src\impl_methods.rs`

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
impl Person {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 8

```rust
    fn new(name: String, age: u32) -> Person {
```

Defines the helper function ``new``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 9

```rust
        Person { name, age }
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    fn get_name(&self) -> &str {
```

Defines the helper function ``get_name``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 13

```rust
        &self.name
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 16

```rust
    fn set_name(&mut self, name: String) {
```

Defines the helper function ``set_name``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 17

```rust
        self.name = name;
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
    fn birthday(&mut self) {
```

Defines the helper function ``birthday``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 21

```rust
        self.age += 1;
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 25

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 26

```rust
    println!("\n2. impl methods");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 28

```rust
    let mut person = Person::new(String::from("Jane"), 25);
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 30

```rust
    println!("Before update: {:?}", person);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 31

```rust
    println!("Read name using &self: {}", person.get_name());
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 33

```rust
    person.set_name(String::from("Bob"));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 34

```rust
    person.birthday();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 36

```rust
    println!("After update using &mut self: {:?}", person);
```

Prints this value so running cargo run shows the behavior of the current lesson.
