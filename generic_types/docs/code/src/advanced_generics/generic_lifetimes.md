# generic_lifetimes.rs

Source: `src/advanced_generics/generic_lifetimes.rs`

## What This Code Is Used For

This code demonstrates generics combined with lifetimes.

## Why Lifetimes Are Needed

The struct stores a reference to generic data. Rust needs a lifetime to know how long that borrowed value is valid.

## Advantages

- Avoids cloning or owning data unnecessarily.
- Makes borrowing rules explicit.

## Disadvantages

- Adds lifetime syntax.
- Borrowed structs can be harder to move around than owned structs.

## When Not To Use It

Own the value directly when the struct should not depend on external data.

## What To Notice In The Code

`BorrowedValue<'a, T>` stores `&'a T`, not `T`.
