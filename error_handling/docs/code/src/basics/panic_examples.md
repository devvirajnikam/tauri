# panic_examples.rs

Source: `src/basics/panic_examples.rs`

## What This Code Is Used For

This code demonstrates `panic!` for unrecoverable problems.

## Why `panic!` Is Chosen

The example treats missing required config as a programming/setup failure rather than normal user input.

## Advantages

- Stops immediately when the program cannot safely continue.
- Makes impossible states obvious.

## Disadvantages

- Crashes the current thread.
- Not suitable for recoverable business errors.

## When Not To Use It

Use `Result` when the caller can recover, retry, show a message, or use a fallback.

## What To Notice In The Code

The panic path is not called during the demo, so `cargo run` continues.
