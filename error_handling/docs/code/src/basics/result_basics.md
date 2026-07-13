# result_basics.rs

Source: `src/basics/result_basics.rs`

## What This Code Is Used For

This code demonstrates `Result<T, E>` with a division function.

## Why `Result` Is Chosen

Division by zero is an expected failure case. `Result` lets the function return either a valid answer or an error message.

## Advantages

- Forces callers to handle success and failure.
- Avoids crashing for expected problems.
- Makes the error path visible in the function signature.

## Disadvantages

- Requires explicit matching or propagation.
- `String` errors are simple but not easy to match by error type.

## When Not To Use It

Use `panic!` only for bugs or impossible states, not normal invalid input.

## What To Notice In The Code

`divide` returns `Result<i32, String>`. The caller uses `match` to handle both `Ok` and `Err`.
