# main.rs

Source: `src/main.rs`

## What This Code Is Used For

`main.rs` runs all error handling lessons in order.

## Why This Structure Is Chosen

The entry point only coordinates modules. Each error topic stays in its own file.

## Advantages

- Easy to run all examples with `cargo run`.
- Keeps beginner, practical, and advanced sections separate.
- Makes future topics easy to add.

## Disadvantages

- Requires understanding Rust module wiring.

## When Not To Use It

For a tiny single-example project, one file would be enough.

## What To Notice In The Code

The project uses modern module files such as `basics.rs`, not old `mod.rs`.
