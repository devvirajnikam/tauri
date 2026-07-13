# arrays_and_slices.rs

Source: `src/common/arrays_and_slices.rs`

## What This Code Is Used For

This code demonstrates fixed arrays and borrowed slices.

## Why Arrays And Slices Are Chosen

Arrays are useful when the length is fixed. Slices are useful when a function should work with part of a collection without taking ownership.

## Advantages

- Arrays have fixed size known at compile time.
- Slices work with arrays and Vec values.
- Borrowing slices avoids unnecessary allocation or copying.

## Disadvantages

- Arrays cannot grow.
- Slices do not own data, so their lifetime depends on the original collection.
- Incorrect range indexing can panic.

## When Not To Use It

- Use `Vec` when the collection must grow.
- Use `String` or `&str` for text-specific data.

## What To Notice In The Code

`print_slice` accepts `&[i32]`, so it can read borrowed values from an array without owning them.
