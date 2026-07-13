# const_generics.rs

Source: `src/advanced_generics/const_generics.rs`

## What This Code Is Used For

This code demonstrates const generics.

## Why Const Generics Are Chosen

The buffer size is part of the type, so `FixedBuffer<T, const N: usize>` can represent different fixed sizes safely.

## Advantages

- Size known at compile time.
- Useful for arrays, fixed buffers, and embedded-style code.
- Avoids runtime length fields for fixed-size data.

## Disadvantages

- More advanced syntax.
- Different sizes are different types.

## When Not To Use It

Use `Vec<T>` when the size should change at runtime.

## What To Notice In The Code

`FixedBuffer::<i32, 4>` means item type is `i32` and length is `4`.
