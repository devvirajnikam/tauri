# map_err_examples.rs

Source: `src/practical_patterns/map_err_examples.rs`

## What This Code Is Used For

This code demonstrates `map_err`.

## Why `map_err` Is Chosen

It converts the error side of a `Result` while leaving the success value unchanged.

## Advantages

- Adds context to low-level errors.
- Keeps transformation local.
- Works well before using `?`.

## Disadvantages

- Can lose structured error information if converted to String too early.

## When Not To Use It

Use a custom error enum when callers need to match exact error cases.

## What To Notice In The Code

The parse error is rewritten into a port-specific message.
