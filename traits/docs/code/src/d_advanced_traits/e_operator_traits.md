# `src/c_advanced_traits/e_operator_traits.rs`

## What This Code Is Used For

This example shows how custom types can support operators such as `+`.

## Why This Pattern Is Chosen

Rust operators are powered by traits in `std::ops`. Implementing `Add` for
`Money` defines what `first + second` means.

## Advantages

- Makes domain types feel natural to use.
- Keeps operator behavior type-safe.
- Works with Rust's standard operator syntax.

## Disadvantages

- Operator behavior must be obvious, or readers may be surprised.
- Ownership matters because this example consumes both `Money` values.

## When Not To Use It

Avoid operator traits when the operation is not obvious. A named method such as
`merge`, `apply_discount`, or `combine` may communicate intent better.

## What To Notice In The Code

`type Output = Money` tells Rust that adding two `Money` values returns another
`Money`. The `add` method contains the actual addition logic.
