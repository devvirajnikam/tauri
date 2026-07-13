# main.rs

Source: `src/main.rs`

## What This Code Is Used For

`main.rs` runs all generic type examples in order.

## Why This Structure Is Chosen

The entry point only coordinates sections. Each generic topic stays in its own module.

## Advantages

- Keeps the runner small.
- Makes topics easy to add.
- Uses modern Rust module syntax without `mod.rs`.

## Disadvantages

- More files to navigate than a single-file demo.

## When Not To Use It

For one tiny generic function demo, one file is enough.

## What To Notice In The Code

The examples are grouped into basics, practical patterns, and advanced generics.
