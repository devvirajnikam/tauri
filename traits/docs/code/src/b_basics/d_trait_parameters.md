# `src/a_basics/d_trait_parameters.rs`

## What This Code Is Used For

This example shows `impl Trait` in function parameters.

## Why This Pattern Is Chosen

`send_alert` only needs something that can notify. Writing `&impl Notifier`
keeps the function signature short while still enforcing the trait requirement.

## Advantages

- Shorter than writing a named generic type for simple cases.
- Keeps the function focused on behavior.
- Avoids exposing unnecessary generic type names.

## Disadvantages

- Less flexible than named generics when multiple parameters must be the same
  concrete type.
- Can be limiting for complex bounds.

## When Not To Use It

Use named generics when the same type appears in multiple places in the
signature, or when the bounds are complex enough to need a `where` clause.

## What To Notice In The Code

`EmailNotifier` is the concrete type, but `send_alert` does not mention it. The
function works with any value implementing `Notifier`.
