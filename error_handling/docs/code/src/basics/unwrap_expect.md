# unwrap_expect.rs

Source: `src/basics/unwrap_expect.rs`

## What This Code Is Used For

This code demonstrates `unwrap` and `expect`.

## Why These Methods Are Shown

They are common in examples and tests, but they panic when the value is `Err` or `None`.

## Advantages

- Very short.
- Useful when failure is impossible or should fail the test immediately.
- `expect` gives a custom panic message.

## Disadvantages

- Can crash the program.
- Bad for normal user input or recoverable errors.

## When Not To Use It

Avoid `unwrap` and `expect` in production paths where invalid input, missing data, or IO failure can happen.

## What To Notice In The Code

The examples use only safe values, so they do not panic during `cargo run`.
