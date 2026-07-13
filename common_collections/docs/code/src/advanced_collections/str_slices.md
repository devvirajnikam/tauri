# str_slices.rs

Source: `src/advanced_collections/str_slices.rs`

## What This Code Is Used For

This code demonstrates `&str`, a borrowed string slice.

## Why `&str` Is Chosen

Use `&str` when a function only needs to read text and should not take ownership of a `String`.

## Advantages

- No allocation.
- Works with string literals and borrowed `String` values.
- Good for function parameters.

## Disadvantages

- Does not own the text.
- Cannot grow or mutate the text.
- Slice indexes must be valid UTF-8 boundaries.

## When Not To Use It

Use `String` when the function needs to own or modify the text.

## What To Notice In The Code

`print_label` accepts `&str`, so it can read both string literals and borrowed `String` values.
