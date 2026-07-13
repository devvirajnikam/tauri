# reusable_reader.rs

Source: `src/reusable_reader.rs`

## What This Code Is Used For

This code creates a reusable helper for prompting and reading input.

## Why A Helper Function Is Chosen

Prompting, flushing stdout, reading a line, trimming, and returning a `Result` are repeated tasks. A helper keeps that logic in one place.

## Advantages

- Reduces repeated input code.
- Handles prompt flushing correctly.
- Returns `Result`, so callers can decide how to handle errors.

## Disadvantages

- Slightly more abstraction than a beginner one-off example.
- Always trims and owns the returned `String`, which may not fit every case.

## When Not To Use It

Do not create a helper if only one tiny function reads input once.

## What To Notice In The Code

`stdout().flush()?` is important because `print!` does not automatically flush the prompt before waiting for input.
