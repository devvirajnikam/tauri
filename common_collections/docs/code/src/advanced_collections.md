# advanced_collections.rs

Source: `src/advanced_collections.rs`

## What This Code Is Used For

This module groups advanced collection-like types and collection processing patterns.

## Why This Structure Is Chosen

The basic collection modules cover the core standard collections. This section covers related tools that show up in real Rust: borrowed text, fixed owned slices, shared ownership, interior mutability, iterator adapters, custom collection traits, and third-party collection choices.

## Advantages

- Keeps advanced concepts separate from beginner collection types.
- Makes the project easier to grow.
- Shows how collections connect with ownership and borrowing.

## Disadvantages

- Some topics are not collections in the strict sense, such as `Rc<T>` or `Option<T>`.
- These examples are intentionally small and do not replace full production patterns.

## When Not To Use It

Do not start here if you are still learning `Vec`, `HashMap`, and `HashSet`. Learn the common collections first.

## What To Notice In The Code

The `run()` function calls every advanced module in order, continuing the lesson numbering after the practical examples.
