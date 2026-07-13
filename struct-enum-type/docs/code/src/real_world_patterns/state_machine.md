# state_machine.rs

Source: `src\real_world_patterns\state_machine.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 8

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 9

```rust
struct Order {
```

Defines the struct ``Order``. A struct is chosen because this type needs named fields with clear meaning.

### Line 10

```rust
    id: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 11

```rust
    state: OrderState,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 14

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 15

```rust
enum OrderState {
```

Defines the enum ``OrderState``. An enum is chosen when the value must be exactly one case from a known set.

### Line 16

```rust
    Draft,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 17

```rust
    Submitted,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 18

```rust
    Paid { transaction_id: String },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 19

```rust
    Cancelled { reason: String },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 22

```rust
impl Order {
```

Starts an implementation block so methods stay attached to the type they operate on.

### Line 23

```rust
    fn submit(&mut self) {
```

Defines the helper function ``submit``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 24

```rust
        if matches!(self.state, OrderState::Draft) {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 25

```rust
            self.state = OrderState::Submitted;
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 29

```rust
    fn pay(&mut self, transaction_id: String) {
```

Defines the helper function ``pay``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 30

```rust
        if matches!(self.state, OrderState::Submitted) {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 31

```rust
            self.state = OrderState::Paid { transaction_id };
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 35

```rust
    fn cancel(&mut self, reason: String) {
```

Defines the helper function ``cancel``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 36

```rust
        if !matches!(self.state, OrderState::Paid { .. }) {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 37

```rust
            self.state = OrderState::Cancelled { reason };
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 42

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 43

```rust
    println!("\n29. Enums as state machines");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 45

```rust
    let mut order = Order {
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 46

```rust
        id: 101,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 47

```rust
        state: OrderState::Draft,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 50

```rust
    println!("Initial order: {:?}", order);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 51

```rust
    println!("Order id: {}", order.id);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 53

```rust
    order.submit();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 54

```rust
    println!("After submit: {:?}", order);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 56

```rust
    order.pay(String::from("TXN-123"));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 57

```rust
    println!("After pay: {:?}", order);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 59

```rust
    match &order.state {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 60

```rust
        OrderState::Paid { transaction_id } => {
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 61

```rust
            println!("Paid transaction id: {}", transaction_id);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 63

```rust
        OrderState::Cancelled { reason } => println!("Cancelled reason: {}", reason),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 64

```rust
        OrderState::Draft => println!("Order is still draft"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 65

```rust
        OrderState::Submitted => println!("Order is submitted"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 68

```rust
    let mut cancelled_order = Order {
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 69

```rust
        id: 102,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 70

```rust
        state: OrderState::Draft,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 72

```rust
    cancelled_order.cancel(String::from("Customer changed mind"));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 73

```rust
    println!("Cancelled order: {:?}", cancelled_order);
```

Prints this value so running cargo run shows the behavior of the current lesson.
