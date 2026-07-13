# nested_types.rs

Source: `src\advanced_topics\nested_types.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 2

```rust
struct Address {
```

Defines the struct ``Address``. A struct is chosen because this type needs named fields with clear meaning.

### Line 3

```rust
    city: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 4

```rust
    country: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 7

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 8

```rust
struct Customer {
```

Defines the struct ``Customer``. A struct is chosen because this type needs named fields with clear meaning.

### Line 9

```rust
    name: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 10

```rust
    address: Address,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 11

```rust
    status: CustomerStatus,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 14

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 15

```rust
enum CustomerStatus {
```

Defines the enum ``CustomerStatus``. An enum is chosen when the value must be exactly one case from a known set.

### Line 16

```rust
    Active,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 17

```rust
    Blocked { reason: String },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 21

```rust
    println!("\n15. Nested structs and enums");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 23

```rust
    let active_customer = Customer {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 24

```rust
        name: String::from("Ravi"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 25

```rust
        address: Address {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 26

```rust
            city: String::from("Mumbai"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 27

```rust
            country: String::from("India"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 29

```rust
        status: CustomerStatus::Active,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 32

```rust
    let blocked_customer = Customer {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 33

```rust
        name: String::from("Nina"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 34

```rust
        address: Address {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 35

```rust
            city: String::from("Pune"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 36

```rust
            country: String::from("India"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 38

```rust
        status: CustomerStatus::Blocked {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 39

```rust
            reason: String::from("Payment pending"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 43

```rust
    println!("{:?}", active_customer);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 44

```rust
    println!(
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 45

```rust
        "Active customer fields -> name: {}, city: {}, country: {}, status: {:?}",
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 46

```rust
        active_customer.name,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 47

```rust
        active_customer.address.city,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 48

```rust
        active_customer.address.country,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 49

```rust
        active_customer.status
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 51

```rust
    match blocked_customer.status {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 52

```rust
        CustomerStatus::Active => println!("{} is active", blocked_customer.name),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 53

```rust
        CustomerStatus::Blocked { reason } => {
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 54

```rust
            println!("{} is blocked: {}", blocked_customer.name, reason);
```

Prints this value so running cargo run shows the behavior of the current lesson.
