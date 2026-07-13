# custom_error_enum.rs

Source: `src/practical_patterns/custom_error_enum.rs`

## What This Code Is Used For

This code demonstrates a custom error enum.

## Why A Custom Enum Is Chosen

The function has multiple known business failure cases. An enum names those cases directly.

## Advantages

- Callers can match exact error variants.
- Variants can carry specific data.
- Better API than returning plain strings.

## Disadvantages

- More code than `String`.
- Manual `Display` implementation adds boilerplate without helper crates.

## When Not To Use It

Use simple `String` errors only for tiny demos or app-level messages where matching is not needed.

## What To Notice In The Code

`CreateAccountError` has two variants, and `Display` controls the user-facing message.
