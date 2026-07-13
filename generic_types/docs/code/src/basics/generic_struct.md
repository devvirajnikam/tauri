# generic_struct.rs

Source: `src/basics/generic_struct.rs`

## What This Code Is Used For

This code demonstrates a generic struct.

## Why A Generic Struct Is Chosen

`BoxedValue<T>` has the same wrapper shape regardless of whether it stores a number, string, or another type.

## Advantages

- Reuses one struct shape for many data types.
- Keeps wrapper behavior consistent.
- Avoids creating separate `NumberBox`, `TextBox`, etc.

## Disadvantages

- The generic type may need trait bounds once methods require behavior like printing or cloning.

## When Not To Use It

Use a concrete struct when the fields always have one specific type.

## What To Notice In The Code

Rust infers `T` from the value assigned to the `value` field.
