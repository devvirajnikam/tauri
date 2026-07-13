# validation_errors.rs

Source: `src/practical_patterns/validation_errors.rs`

## What This Code Is Used For

This code returns multiple validation errors from one form.

## Why `Result<T, Vec<E>>` Is Chosen

Form validation often needs to show all problems at once instead of stopping at the first failure.

## Advantages

- Reports all validation errors.
- Keeps valid form data in the `Ok` branch.
- Error enum keeps validation cases explicit.

## Disadvantages

- More complex than returning one error.
- The caller must handle a list of errors.

## When Not To Use It

Use a single error when later checks depend on earlier checks passing.

## What To Notice In The Code

Errors are collected in a Vec, and the function returns `Ok(form)` only when the Vec is empty.
