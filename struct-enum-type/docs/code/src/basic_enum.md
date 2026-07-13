# basic_enum.rs

Source: `src\basic_enum.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
enum Role {
```

Defines the enum ``Role``. An enum is chosen when the value must be exactly one case from a known set.

### Line 3

```rust
    Admin,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 4

```rust
    User,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 5

```rust
    Guest,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 8

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 9

```rust
struct Person {
```

Defines the struct ``Person``. A struct is chosen because this type needs named fields with clear meaning.

### Line 10

```rust
    name: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 11

```rust
    role: Role,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 14

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 15

```rust
    println!("\n3. Basic enum");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 17

```rust
    let admin = Person {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 18

```rust
        name: String::from("Asha"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 19

```rust
        role: Role::Admin,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 22

```rust
    let user = Person {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 23

```rust
        name: String::from("Ravi"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 24

```rust
        role: Role::User,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 27

```rust
    let guest = Person {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 28

```rust
        name: String::from("Meera"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 29

```rust
        role: Role::Guest,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 32

```rust
    println!("{:?}", admin);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 33

```rust
    println!("{:?}", user);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 34

```rust
    println!("{:?}", guest);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 35

```rust
    println!("{} has role {:?}", admin.name, admin.role);
```

Prints this value so running cargo run shows the behavior of the current lesson.
