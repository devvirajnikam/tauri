# visibility.rs

Source: `src\advanced_topics\visibility.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
pub struct Account {
```

Defines the public struct ``Account``. A struct is chosen because named fields should travel together as one domain value.

### Line 2

```rust
    pub owner: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 3

```rust
    balance: i32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 6

```rust
impl Account {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 7

```rust
    pub fn new(owner: String, balance: i32) -> Account {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 8

```rust
        Account { owner, balance }
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 11

```rust
    pub fn deposit(&mut self, amount: i32) {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
        self.balance += amount;
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 15

```rust
    pub fn balance(&self) -> i32 {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
        self.balance
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 21

```rust
    println!("\n14. Visibility with pub");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 23

```rust
    let mut account = Account::new(String::from("Asha"), 100);
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 24

```rust
    account.deposit(50);
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 26

```rust
    println!("Owner is public: {}", account.owner);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 27

```rust
    println!(
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 28

```rust
        "Balance is private, read through method: {}",
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 29

```rust
        account.balance()
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
