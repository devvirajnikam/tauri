# trait_bounds.rs

Source: `src/practical_patterns/trait_bounds.rs`

## What This Code Is Used For

This code demonstrates trait bounds.

## Why Trait Bounds Are Chosen

The function wants to print a generic value. `T: Display` tells Rust that any accepted type must support `{}` formatting.

## Advantages

- Keeps the function generic but still guarantees required behavior.
- Compiler catches unsupported types.

## Disadvantages

- More constraints mean fewer types can use the function.

## When Not To Use It

Do not add bounds until the function actually needs that behavior.

## What To Notice In The Code

`print_twice` accepts strings and numbers because both implement `Display`.
