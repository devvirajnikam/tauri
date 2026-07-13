# print_and_flush.rs

Source: `src/print_and_flush.rs`

## What This Code Is Used For

This code demonstrates output without an automatic newline and manual stdout flushing.

## Why `print!` And `flush` Are Chosen

`print!` is useful for prompts or inline output. `flush` is needed when the user must see the text immediately.

## Advantages

- Good for prompts.
- Lets output continue on the same line.
- Makes buffering behavior explicit.

## Disadvantages

- Easy to forget flushing.
- More code than `println!`.

## When Not To Use It

Use `println!` when a normal full line of output is enough.

## What To Notice In The Code

The code calls `io::stdout().flush()` after `print!` so the partial line appears immediately.
