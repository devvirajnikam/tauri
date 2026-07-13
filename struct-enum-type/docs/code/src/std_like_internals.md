# std_like_internals.rs

Source: `src\std_like_internals.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 10

```rust
mod my_box;
```

Declares the ``my_box`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 11

```rust
mod my_cell;
```

Declares the ``my_cell`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 12

```rust
mod my_control_flow;
```

Declares the ``my_control_flow`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 13

```rust
mod my_hash_map;
```

Declares the ``my_hash_map`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 14

```rust
mod my_ip_addr;
```

Declares the ``my_ip_addr`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 15

```rust
mod my_option;
```

Declares the ``my_option`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 16

```rust
mod my_ordering;
```

Declares the ``my_ordering`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 17

```rust
mod my_poll;
```

Declares the ``my_poll`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 18

```rust
mod my_range;
```

Declares the ``my_range`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 19

```rust
mod my_result;
```

Declares the ``my_result`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 20

```rust
mod my_string;
```

Declares the ``my_string`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 21

```rust
mod my_vec;
```

Declares the ``my_vec`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 23

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 24

```rust
    println!("\nStd-like internals recreated for learning");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 26

```rust
    my_option::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 27

```rust
    my_result::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 28

```rust
    my_ordering::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 29

```rust
    my_control_flow::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 30

```rust
    my_poll::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 31

```rust
    my_range::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 32

```rust
    my_box::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 33

```rust
    my_vec::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 34

```rust
    my_string::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 35

```rust
    my_hash_map::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 36

```rust
    my_ip_addr::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 37

```rust
    my_cell::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
