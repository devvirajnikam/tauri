# generic_enum.rs

Source: `src/basics/generic_enum.rs`

## What This Code Is Used For

This code demonstrates a generic enum.

## Why A Generic Enum Is Chosen

`ApiResult<T, E>` lets the success and failure variants carry different caller-chosen types.

## Advantages

- Good for response, state, and result-like shapes.
- Success and error payload types stay flexible.
- Similar idea to Rust's built-in `Result<T, E>`.

## Disadvantages

- Type annotations may be needed when Rust cannot infer both `T` and `E`.

## When Not To Use It

Use a concrete enum when every variant always carries fixed data types.

## What To Notice In The Code

`ApiResult<String, String>` means success carries `String` and failure also carries `String`.
