# `src/a_basics/c_trait_bounds.rs`

## What This Code Is Used For

This example shows how generic functions can require specific trait behavior.

## Why This Pattern Is Chosen

`print_anything` should work with any type that knows how to print itself. The
trait bound `T: Printable` tells Rust that calling `value.print()` is valid.

## Advantages

- Generic code stays type-safe.
- The function can accept many types without using `dyn Trait`.
- Rust can often optimize this style very well.

## Disadvantages

- Generic signatures can become harder to read as bounds grow.
- Each concrete type produces specialized compiled code, which can increase
  binary size in large projects.

## When Not To Use It

If you need a collection containing many different concrete types at runtime, a
trait object such as `Box<dyn Trait>` may fit better.

## What To Notice In The Code

The function does not know about `Invoice` directly. It only knows that `T`
implements `Printable`, which is enough to call `print`.
