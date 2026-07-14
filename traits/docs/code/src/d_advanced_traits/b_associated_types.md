# `src/c_advanced_traits/b_associated_types.rs`

## What This Code Is Used For

This example shows how a trait can define a related type chosen by each
implementation.

## Why This Pattern Is Chosen

A loader has one main output type. Putting that output inside the trait as
`type Output` makes the relationship between the implementation and its result
clear.

## Advantages

- Keeps related types attached to the trait implementation.
- Avoids repeating generic type parameters at every use site.
- Common in iterator-like and loader-like designs.

## Disadvantages

- Each implementation chooses one output type for that trait.
- Can be harder for beginners than a simple generic parameter.

## When Not To Use It

Use generic trait parameters when one type needs to implement the same trait for
multiple different input or output types.

## What To Notice In The Code

`ConfigLoader` sets `type Output = String`. After that, `load` returns
`Self::Output`, which means `String` for this implementation.
