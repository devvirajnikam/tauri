# where_clause.rs

Source: `src/practical_patterns/where_clause.rs`

## What This Code Is Used For

This code demonstrates `where` clauses.

## Why `where` Is Chosen

When multiple generic parameters have multiple bounds, putting bounds after `where` keeps the function signature readable.

## Advantages

- Cleaner signatures.
- Easier to read complex bounds.
- Common in real generic APIs.

## Disadvantages

- More syntax to learn.

## When Not To Use It

Inline bounds are fine for short examples like `fn print<T: Display>(...)`.

## What To Notice In The Code

Both `T` and `U` must implement `Display` and `Debug`.
