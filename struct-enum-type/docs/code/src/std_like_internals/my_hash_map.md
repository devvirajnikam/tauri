# my_hash_map.rs

Source: `src\std_like_internals\my_hash_map.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 8

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 9

```rust
struct MyHashMap<K, V> {
```

Defines the generic struct ``MyHashMap``. Generics are chosen so the same wrapper or container can work with different inner data types.

### Line 10

```rust
    entries: Vec<(K, V)>,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
impl<K: PartialEq, V> MyHashMap<K, V> {
```

Starts a generic implementation block. The type parameters make these methods available for many concrete versions of the type.

### Line 14

```rust
    fn new() -> MyHashMap<K, V> {
```

Defines the helper function ``new``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 15

```rust
        MyHashMap {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
            entries: Vec::new(),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
    fn insert(&mut self, key: K, value: V) {
```

Defines the helper function ``insert``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 21

```rust
        for (existing_key, existing_value) in &mut self.entries {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 22

```rust
            if *existing_key == key {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 23

```rust
                *existing_value = value;
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 24

```rust
                return;
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 28

```rust
        self.entries.push((key, value));
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 31

```rust
    fn get(&self, key: &K) -> Option<&V> {
```

Defines the helper function ``get``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 32

```rust
        for (existing_key, existing_value) in &self.entries {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 33

```rust
            if existing_key == key {
```

Starts a conditional branch because the behavior depends on a boolean check.

### Line 34

```rust
                return Some(existing_value);
```

Returns early because the function already knows the final answer or error at this point.

### Line 38

```rust
        None
```

Represents an optional value that is missing, avoiding null.

### Line 42

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 43

```rust
    println!("\n53. Recreated HashMap-like struct");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 45

```rust
    let mut quantities = MyHashMap::new();
```

Creates a mutable local variable. mut is required because this value is changed later in the example.

### Line 46

```rust
    quantities.insert(String::from("Keyboard"), 10);
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 47

```rust
    quantities.insert(String::from("Mouse"), 5);
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 48

```rust
    quantities.insert(String::from("Keyboard"), 12);
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 50

```rust
    println!("{:?}", quantities);
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 51

```rust
    println!(
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 52

```rust
        "Keyboard quantity: {:?}",
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 53

```rust
        quantities.get(&String::from("Keyboard"))
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
