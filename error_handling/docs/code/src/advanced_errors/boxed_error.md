# boxed_error.rs

Source: `src/advanced_errors/boxed_error.rs`

## What This Code Is Used For

This code demonstrates `Box<dyn Error>`.

## Why It Is Chosen

It allows one function to return many possible error types behind a trait object.

## Advantages

- Convenient for examples and application-level code.
- Works smoothly with `?`.
- Avoids defining a custom enum for every small app flow.

## Disadvantages

- Callers lose easy access to exact error variants.
- Less ideal for libraries with stable APIs.

## When Not To Use It

Use a custom error enum when callers need to match specific error cases.

## What To Notice In The Code

The parse error is automatically converted into the boxed error when `?` is used.
