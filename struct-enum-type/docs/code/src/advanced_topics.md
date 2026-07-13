# advanced_topics.rs

Source: `src\advanced_topics.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 1

```rust
mod associated_functions;
```

Declares the ``associated_functions`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 2

```rust
mod derived_traits;
```

Declares the ``derived_traits`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 3

```rust
mod destructuring;
```

Declares the ``destructuring`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 4

```rust
mod if_while_let;
```

Declares the ``if_while_let`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 5

```rust
mod lifetimes;
```

Declares the ``lifetimes`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 6

```rust
mod nested_types;
```

Declares the ``nested_types`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 7

```rust
mod option_result_helpers;
```

Declares the ``option_result_helpers`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 8

```rust
mod pattern_guards;
```

Declares the ``pattern_guards`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 9

```rust
mod recursive_enums;
```

Declares the ``recursive_enums`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 10

```rust
mod self_methods;
```

Declares the ``self_methods`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 11

```rust
mod tuple_unit_structs;
```

Declares the ``tuple_unit_structs`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 12

```rust
mod visibility;
```

Declares the ``visibility`` module. This modern Rust module syntax points to a sibling file or folder entry without using old mod.rs style.

### Line 14

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 15

```rust
    println!("\nAdvanced struct and enum topics");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 17

```rust
    tuple_unit_structs::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 18

```rust
    destructuring::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 19

```rust
    if_while_let::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
    self_methods::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 21

```rust
    associated_functions::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 22

```rust
    visibility::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 23

```rust
    nested_types::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 24

```rust
    lifetimes::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 25

```rust
    derived_traits::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 26

```rust
    recursive_enums::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 27

```rust
    pattern_guards::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 28

```rust
    option_result_helpers::run();
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
