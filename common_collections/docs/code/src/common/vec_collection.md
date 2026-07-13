# vec_collection.rs

Source: `src/common/vec_collection.rs`

## What This Code Is Used For

This code demonstrates `Vec<T>`, Rust's growable list collection. Use it for ordered data where the number of items can change.

## Why Vec Is Chosen

`Vec<T>` is usually the first collection to consider when you need a list. It is simple, fast, ordered, and efficient for adding/removing items at the end.

## Advantages

- Keeps insertion order.
- Fast indexing by position.
- Efficient `push` at the end.
- Works very well with iterators like `.iter()`, `.map()`, and `.collect()`.

## Disadvantages

- Removing from the front is inefficient because items must shift.
- Searching by value is linear unless sorted or indexed separately.
- Indexing with `vec[index]` can panic if the index is invalid.

## When Not To Use It

- Use `VecDeque` for queue behavior.
- Use `HashMap` for fast lookup by key.
- Use `HashSet` if uniqueness is the main requirement.

## What To Notice In The Code

The example uses `push` to add values, `get(0)` to safely read by index, `&scores` to iterate without moving the Vec, and `.iter().map().collect()` to build a transformed Vec.
