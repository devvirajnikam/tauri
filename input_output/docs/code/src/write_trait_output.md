# write_trait_output.rs

Source: `src/write_trait_output.rs`

## What This Code Is Used For

This code demonstrates writing output through the `Write` trait.

## Why `Write` Is Chosen

`Write` makes output destination-independent. The same function can write to stdout, a file, a memory buffer, or a test buffer.

## Advantages

- Easier to test than direct `println!`.
- Reusable for many output destinations.
- Returns `io::Result`, so write failures can be handled.

## Disadvantages

- More abstract than direct printing.
- Requires handling IO errors.

## When Not To Use It

Use `println!` for simple demo output where testability and destination flexibility do not matter.

## What To Notice In The Code

`write_report` accepts `impl Write`, writes into a `Vec<u8>`, and then converts the buffer into text for display.
