# my_ip_addr.rs

Source: `src\std_like_internals\my_ip_addr.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 8

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 9

```rust
enum MyIpAddr {
```

Defines the enum ``MyIpAddr``. An enum is chosen when the value must be exactly one case from a known set.

### Line 10

```rust
    V4(u8, u8, u8, u8),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 11

```rust
    V6(String),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
fn print_ip(ip: MyIpAddr) {
```

Defines the helper function ``print_ip``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 15

```rust
    match ip {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 16

```rust
        MyIpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 17

```rust
        MyIpAddr::V6(address) => println!("IPv6: {}", address),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 21

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 22

```rust
    println!("\n54. Recreated IpAddr enum");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 24

```rust
    print_ip(MyIpAddr::V4(127, 0, 0, 1));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 25

```rust
    print_ip(MyIpAddr::V6(String::from("::1")));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
