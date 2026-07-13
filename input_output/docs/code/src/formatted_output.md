# formatted_output.rs

Source: `src/formatted_output.rs`

## What This Code Is Used For

This code demonstrates Rust's formatting options in output macros.

## Why Formatting Macros Are Chosen

Formatting macros let the program print values, debug data, alignment, and decimal precision without manually constructing strings.

## Advantages

- Clear syntax for common display needs.
- Supports `Display` with `{}` and `Debug` with `{:?}`.
- Useful for reports, logs, and CLI output.

## Disadvantages

- Format strings can become hard to read if too complex.
- `Debug` output is not always user-friendly.

## When Not To Use It

Use dedicated rendering or serialization when output must be a strict machine-readable format.

## What To Notice In The Code

The example shows `{}`, `{:?}`, `{:#?}`, `{:.2}`, and `{:>5}`.
