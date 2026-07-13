# main.rs

Source: `src/main.rs`

## What This Code Is Used For

`main.rs` is the entry point of the learning project. It runs the collection lessons in order.

## Why This Structure Is Better

The file stays small and only coordinates modules. The actual examples live in focused files, so each collection topic is easy to find and understand.

## Advantages

- Keeps the project organized.
- Makes it easy to add more collection topics later.
- Avoids putting all examples into one large file.

## Disadvantages

- You need to understand Rust module wiring.
- There are more files to navigate.

## What To Notice

`mod common;` and `mod practical_patterns;` use modern Rust module syntax. The folders are connected through `common.rs` and `practical_patterns.rs`, not `mod.rs`.
