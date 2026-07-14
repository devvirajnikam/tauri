# `src/main.rs`

## What This Code Is Used For

This file is the entry point of the traits project. It connects the topic groups
and runs them in a learning-friendly order.

## Why This Pattern Is Chosen

The examples are split into `basics`, `practical_patterns`, and
`advanced_traits` so each concept can stay small. `main.rs` only coordinates the
modules instead of containing lesson code directly.

## Advantages

- Easy to run the full project with one `cargo run`.
- Easy to find examples by topic.
- Keeps the entry point simple and readable.

## Disadvantages

- You need to jump between files to study every example.
- Very small examples can feel spread out across many modules.

## When Not To Use It

For a tiny one-file experiment, keeping everything in `main.rs` can be simpler.
Once the project becomes a learning reference, separate modules are easier.

## What To Notice In The Code

`mod basics;`, `mod practical_patterns;`, and `mod advanced_traits;` use modern
Rust module files. The child modules live in folders beside the parent file, not
inside older `mod.rs` files.
