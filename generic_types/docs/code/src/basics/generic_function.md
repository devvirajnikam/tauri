# generic_function.rs

Source: `src/basics/generic_function.rs`

## What This Code Is Used For

This code demonstrates a generic function.

## Why A Generic Function Is Chosen

The `first` function can return the first item from a slice of any type. Writing one function is better than duplicating the same logic for numbers, strings, and other types.

## Advantages

- Removes repeated code.
- Works with many input types.
- Keeps the function focused on behavior, not a specific concrete type.

## Disadvantages

- The function can only use operations available for all possible `T`.
- Error messages can be harder for beginners.

## When Not To Use It

Use a concrete function when the logic only makes sense for one type.

## What To Notice In The Code

`first<T>` works for both `[i32]` and `[&str]` because it only borrows and returns an item.
