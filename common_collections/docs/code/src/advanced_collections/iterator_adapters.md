# iterator_adapters.rs

Source: `src/advanced_collections/iterator_adapters.rs`

## What This Code Is Used For

This code demonstrates iterator adapters such as `filter`, `map`, `fold`, and `enumerate`.

## Why Iterator Adapters Are Chosen

They express collection processing as a pipeline and avoid manual index loops.

## Advantages

- Clear transformation flow.
- Works across many collection types.
- Often avoids temporary mutable variables.

## Disadvantages

- Long chains can become hard to read.
- Borrowing rules inside closures can confuse beginners.

## When Not To Use It

Use a normal loop when the logic has many branches or side effects.

## What To Notice In The Code

The code filters even numbers, squares them, collects them into a Vec, folds numbers into a total, and labels values with `enumerate`.
