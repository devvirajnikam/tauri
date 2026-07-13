# search_query.rs

Source: `src\real_world_patterns\search_query.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 10

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 11

```rust
enum SortField {
```

Defines the enum ``SortField``. An enum is chosen when the value must be exactly one case from a known set.

### Line 12

```rust
    Name,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
    CreatedAt,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 17

```rust
enum SortDirection {
```

Defines the enum ``SortDirection``. An enum is chosen when the value must be exactly one case from a known set.

### Line 18

```rust
    Asc,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 19

```rust
    Desc,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 22

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 23

```rust
struct SearchQuery {
```

Defines the struct ``SearchQuery``. A struct is chosen because this type needs named fields with clear meaning.

### Line 24

```rust
    text: String,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 25

```rust
    page: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 26

```rust
    per_page: u32,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 27

```rust
    sort_field: SortField,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 28

```rust
    sort_direction: SortDirection,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 31

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 32

```rust
    println!("\n42. Search query structs");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 34

```rust
    let query = SearchQuery {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 35

```rust
        text: String::from("keyboard"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 36

```rust
        page: 1,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 37

```rust
        per_page: 20,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 38

```rust
        sort_field: SortField::CreatedAt,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 39

```rust
        sort_direction: SortDirection::Desc,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 42

```rust
    println!("{:?}", query);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 43

```rust
    println!(
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 44

```rust
        "Query fields -> text: {}, page: {}, per_page: {}, sort: {:?} {:?}",
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 45

```rust
        query.text, query.page, query.per_page, query.sort_field, query.sort_direction
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 48

```rust
    let name_ascending = SearchQuery {
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 49

```rust
        text: String::from("mouse"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 50

```rust
        page: 2,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 51

```rust
        per_page: 10,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 52

```rust
        sort_field: SortField::Name,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 53

```rust
        sort_direction: SortDirection::Asc,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 56

```rust
    println!("Another query: {:?}", name_ascending);
```

Prints this value so running cargo run shows the behavior of the current lesson.
