# money_currency.rs

Source: `src\real_world_patterns\money_currency.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
use std::fmt;
```

Imports a type, trait, or module path so this file can use a shorter name instead of repeating the full path.

### Line 11

```rust
#[derive(Debug, Clone, Copy)]
```

Asks the compiler to generate these trait implementations: Debug, Clone, Copy. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 12

```rust
enum Currency {
```

Defines the enum ``Currency``. An enum is chosen when the value must be exactly one case from a known set.

### Line 13

```rust
    Inr,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
    Usd,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 17

```rust
#[derive(Debug, Clone, Copy)]
```

Asks the compiler to generate these trait implementations: Debug, Clone, Copy. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 18

```rust
struct Money {
```

Defines the struct ``Money``. A struct is chosen because this type needs named fields with clear meaning.

### Line 19

```rust
    amount_in_minor_units: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 20

```rust
    currency: Currency,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 23

```rust
impl Money {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 24

```rust
    fn new(amount_in_minor_units: u32, currency: Currency) -> Money {
```

Defines the helper function ``new``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 25

```rust
        Money {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 26

```rust
            amount_in_minor_units,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 27

```rust
            currency,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 32

```rust
impl fmt::Display for Money {
```

Implements ``fmt::Display`` for ``Money``. This is chosen when the type should support an existing behavior contract.

### Line 33

```rust
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
```

Defines the helper function ``fmt``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 34

```rust
        let major = self.amount_in_minor_units / 100;
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 35

```rust
        let minor = self.amount_in_minor_units % 100;
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 36

```rust
        let symbol = match self.currency {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 37

```rust
            Currency::Inr => "INR",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 38

```rust
            Currency::Usd => "USD",
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 41

```rust
        write!(formatter, "{} {}.{:02}", symbol, major, minor)
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 45

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 46

```rust
    println!("\n36. Money structs and currency enums");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 48

```rust
    let invoice_total = Money::new(123_450, Currency::Inr);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 49

```rust
    let subscription_price = Money::new(2_999, Currency::Usd);
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 51

```rust
    println!("Invoice total: {}", invoice_total);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 52

```rust
    println!("Subscription price: {}", subscription_price);
```

Prints this value so running cargo run shows the behavior of the current lesson.
