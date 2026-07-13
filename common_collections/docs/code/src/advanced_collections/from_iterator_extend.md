# from_iterator_extend.rs

Source: `src/advanced_collections/from_iterator_extend.rs`

## What This Code Is Used For

This code demonstrates `FromIterator` and `Extend` on a custom collection wrapper.

## Why These Traits Are Chosen

`FromIterator` powers `collect()`. `Extend` powers adding many items from an iterator.

## Advantages

- Makes custom wrappers feel like normal Rust collections.
- Works naturally with iterator pipelines.
- Keeps construction and extension behavior standardized.

## Disadvantages

- More trait code to understand.
- Overkill for simple one-off wrappers.

## When Not To Use It

Do not implement these traits unless the type really acts like a collection.

## What To Notice In The Code

`Names` can be created with `.collect()` and later extended with `.extend(...)`.
