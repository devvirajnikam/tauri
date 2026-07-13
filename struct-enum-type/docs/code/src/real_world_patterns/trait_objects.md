# trait_objects.rs

Source: `src\real_world_patterns\trait_objects.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
trait Notifier {
```

Defines the trait ``Notifier``. A trait is chosen to describe shared behavior that different structs or enums can implement.

### Line 10

```rust
    fn send(&self, message: &str);
```

Defines the helper function ``send``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 13

```rust
struct EmailNotifier {
```

Defines the struct ``EmailNotifier``. A struct is chosen because this type needs named fields with clear meaning.

### Line 14

```rust
    address: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 17

```rust
struct SmsNotifier {
```

Defines the struct ``SmsNotifier``. A struct is chosen because this type needs named fields with clear meaning.

### Line 18

```rust
    phone_number: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 21

```rust
impl Notifier for EmailNotifier {
```

Implements ``Notifier`` for ``EmailNotifier``. This is chosen when the type should support an existing behavior contract.

### Line 22

```rust
    fn send(&self, message: &str) {
```

Defines the helper function ``send``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 23

```rust
        println!("Email to {}: {}", self.address, message);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 27

```rust
impl Notifier for SmsNotifier {
```

Implements ``Notifier`` for ``SmsNotifier``. This is chosen when the type should support an existing behavior contract.

### Line 28

```rust
    fn send(&self, message: &str) {
```

Defines the helper function ``send``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 29

```rust
        println!("SMS to {}: {}", self.phone_number, message);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 33

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 34

```rust
    println!("\n28. Trait objects with Box<dyn Trait>");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 36

```rust
    let notifiers: Vec<Box<dyn Notifier>> = vec![
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 37

```rust
        Box::new(EmailNotifier {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 38

```rust
            address: String::from("user@example.com"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 40

```rust
        Box::new(SmsNotifier {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 41

```rust
            phone_number: String::from("+91-9999999999"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 45

```rust
    for notifier in notifiers {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 46

```rust
        notifier.send("Your order has shipped");
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
