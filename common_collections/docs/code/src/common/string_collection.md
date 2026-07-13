# string_collection.rs

Source: `src/common/string_collection.rs`

## What This Code Is Used For

This code demonstrates `String`, Rust's owned growable text type.

## Why String Is Chosen

Use `String` when your program owns the text and may need to change it. It is different from `&str`, which is a borrowed string slice.

## Advantages

- Owns its text data.
- Can grow with `push` and `push_str`.
- Works with text methods like `chars()` and `split_whitespace()`.
- Stores valid UTF-8.

## Disadvantages

- Indexing by character position is not direct because UTF-8 characters can use different byte lengths.
- `.len()` returns byte length, not character count.
- Cloning large strings can allocate memory.

## When Not To Use It

- Use `&str` when you only need to borrow text.
- Use `Vec<u8>` for raw bytes that may not be valid UTF-8.

## What To Notice In The Code

The example appends one character with `push`, appends text with `push_str`, iterates characters with `chars()`, and splits words into a `Vec<&str>`.
