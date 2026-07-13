# linked_list.rs

Source: `src/common/linked_list.rs`

## What This Code Is Used For

This code demonstrates `LinkedList<T>`.

## Why LinkedList Is Included

It is included for understanding, not because it is commonly the best choice. In Rust app code, `Vec` or `VecDeque` is usually better.

## Advantages

- Supports list-like front and back operations.
- Can be useful for specific algorithms that need node-style behavior.

## Disadvantages

- Poor cache locality compared with Vec.
- More allocation overhead.
- Rarely faster in normal application code.

## When Not To Use It

- Use `Vec` for most ordered lists.
- Use `VecDeque` for queue behavior.

## What To Notice In The Code

The example shows `push_front`, `push_back`, and iteration by reference.
