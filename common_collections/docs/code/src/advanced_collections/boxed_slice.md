# boxed_slice.rs

Source: `src/advanced_collections/boxed_slice.rs`

## What This Code Is Used For

This code demonstrates `Box<[T]>`, an owned fixed-size slice.

## Why `Box<[T]>` Is Chosen

Use it when a list is built once and should not grow afterward.

## Advantages

- Owns heap data.
- Has no extra Vec capacity.
- Clearly communicates fixed-size ownership.

## Disadvantages

- Cannot push more items.
- Less flexible than Vec.

## When Not To Use It

Use `Vec<T>` when the list needs to grow or shrink often.

## What To Notice In The Code

The Vec is converted with `into_boxed_slice()`, which keeps the data but removes Vec-style growth behavior.
