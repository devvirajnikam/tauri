# permission_roles.rs

Source: `src\real_world_patterns\permission_roles.rs`

This documentation focuses on Rust code lines that affect the example.

## Line-by-line explanation

### Line 9

```rust
#[derive(Debug, PartialEq)]
```

Asks the compiler to generate these trait implementations: Debug, PartialEq. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 10

```rust
enum Role {
```

Defines the enum ``Role``. An enum is chosen when the value must be exactly one case from a known set.

### Line 11

```rust
    Admin,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 12

```rust
    Manager,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 13

```rust
    Viewer,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 16

```rust
#[derive(Debug, PartialEq)]
```

Asks the compiler to generate these trait implementations: Debug, PartialEq. Without derive, you would manually write impl blocks. For Debug, that means writing impl std::fmt::Debug and defining how the value should be formatted. derive is chosen because the standard generated behavior is enough for these lessons.

### Line 17

```rust
enum Permission {
```

Defines the enum ``Permission``. An enum is chosen when the value must be exactly one case from a known set.

### Line 18

```rust
    View,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 19

```rust
    Edit,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 20

```rust
    Delete,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 23

```rust
fn has_permission(role: &Role, permission: &Permission) -> bool {
```

Defines the helper function ``has_permission``. A helper function is chosen to keep one behavior isolated and reusable inside the example.

### Line 24

```rust
    match role {
```

Starts pattern matching. match is chosen because the code should handle the important enum variants or data shapes explicitly.

### Line 25

```rust
        Role::Admin => true,
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 26

```rust
        Role::Manager => matches!(permission, Permission::View | Permission::Edit),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 27

```rust
        Role::Viewer => matches!(permission, Permission::View),
```

Defines a match arm or closure body: the pattern/input is on the left, and the action/result is on the right.

### Line 31

```rust
pub fn run() {
```

Defines the public demo entry function for this module. It is public so the parent module can call it while running all examples from main.rs.

### Line 32

```rust
    println!("\n35. Role and permission enums");
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 34

```rust
    let roles = [Role::Admin, Role::Manager, Role::Viewer];
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 35

```rust
    let permissions = [Permission::View, Permission::Edit, Permission::Delete];
```

Creates a local variable so the value can be named and reused clearly in later lines.

### Line 37

```rust
    for role in roles {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 38

```rust
        for permission in &permissions {
```

Loops over each item from an iterator, which is clearer than manual indexing for this example.

### Line 39

```rust
            println!(
```

Prints this value so running cargo run shows the behavior of the current lesson.

### Line 40

```rust
                "{:?} can {:?}: {}",
```

This meaningful line belongs to the surrounding Rust expression. Read it with the nearby lines to understand the complete operation.

### Line 41

```rust
                role,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 42

```rust
                permission,
```

Creates, transforms, calls, or returns a value as part of the surrounding example.

### Line 43

```rust
                has_permission(&role, permission)
```

Creates, transforms, calls, or returns a value as part of the surrounding example.
