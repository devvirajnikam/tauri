# command_handler.rs

Source: `src\real_world_patterns\command_handler.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
#[derive(Debug)]
```

Asks the compiler to generate these trait implementations: Debug. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 10

```rust
enum UserCommand {
```

Defines the enum ``UserCommand``. An enum is chosen when the value must be exactly one case from a known set.

### Line 11

```rust
    Create { name: String },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    Rename { id: u32, new_name: String },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
    Delete { id: u32 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 14

```rust
    PrintAll,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 17

```rust
fn handle_command(command: UserCommand) {
```

Defines the helper function ``handle_command``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 18

```rust
    match command {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 19

```rust
        UserCommand::Create { name } => println!("Create user named {}", name),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 20

```rust
        UserCommand::Rename { id, new_name } => println!("Rename user {} to {}", id, new_name),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 21

```rust
        UserCommand::Delete { id } => println!("Delete user {}", id),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 22

```rust
        UserCommand::PrintAll => println!("Print all users"),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 26

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 27

```rust
    println!("\n33. Command enums");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 29

```rust
    let commands = [
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 30

```rust
        UserCommand::Create {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 31

```rust
            name: String::from("John"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 33

```rust
        UserCommand::Rename {
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 34

```rust
            id: 1,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 35

```rust
            new_name: String::from("Jane"),
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 37

```rust
        UserCommand::Delete { id: 2 },
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 38

```rust
        UserCommand::PrintAll,
```

Declares a field or typed value. The type annotation makes the expected data shape explicit.

### Line 41

```rust
    for command in commands {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 42

```rust
        handle_command(command);
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
