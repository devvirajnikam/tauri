# cow_examples.rs

Source: `src/advanced_collections/cow_examples.rs`

## What This Code Is Used For

This code demonstrates `Cow<'a, str>`, clone-on-write text.

## Why `Cow` Is Chosen

Use `Cow` when most inputs can be borrowed, but some inputs need an owned cleaned or changed version.

## Advantages

- Avoids allocation when no change is needed.
- Still allows owned data when a change is needed.
- Useful for normalization and parsing APIs.

## Disadvantages

- More complex than returning `String` or `&str`.
- Can make signatures harder for beginners to read.

## When Not To Use It

Use `String` if every call creates owned text anyway. Use `&str` if no call ever needs ownership.

## What To Notice In The Code

`normalize_name` returns borrowed text when no trimming is needed and owned text when trimming changes the input.
