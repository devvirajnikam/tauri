# result_handling.rs

Source: `src/result_handling.rs`

## What This Code Is Used For

This code reads terminal input and handles the `Result` from `read_line`.

## Why Matching `Result` Is Chosen

Reading from stdin can fail. Matching `Ok` and `Err` makes both outcomes explicit.

## Advantages

- No panic on input failure.
- Shows how many bytes were read.
- Makes success and failure behavior clear.

## Disadvantages

- More code than `expect`.
- Repeating this pattern everywhere can become noisy.

## When Not To Use It

Use a helper function when many modules need the same read-and-handle behavior.

## What To Notice In The Code

`read_line` returns `Result<usize, io::Error>`. `usize` is the number of bytes read.
