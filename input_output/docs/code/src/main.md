# main.rs

Source: `src/main.rs`

## What This Code Is Used For

`main.rs` runs all input and output handling examples in order.

## Why This Structure Is Chosen

The entry point only coordinates modules. The actual input and output examples stay in focused files.

## Advantages

- Keeps the runner small.
- Makes each input/output topic easy to find.
- Uses modern Rust module syntax without `mod.rs`.

## Disadvantages

- Running all modules means the program asks for multiple inputs before showing output examples.

## When Not To Use It

For a single quick stdin/stdout experiment, one file can be enough.

## What To Notice In The Code

Each module has a public `run()` function, and `main()` calls them in learning order.
