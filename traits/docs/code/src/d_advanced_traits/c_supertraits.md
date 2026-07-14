# `src/c_advanced_traits/c_supertraits.rs`

## What This Code Is Used For

This example shows a trait that requires another trait.

## Why This Pattern Is Chosen

`PrettyPrint` needs display behavior because its default method prints the value
inside a formatted wrapper. Requiring `Display` guarantees that formatting is
available.

## Advantages

- Builds stronger trait contracts from smaller traits.
- Lets default methods rely on behavior from the parent trait.
- Makes requirements explicit to implementers.

## Disadvantages

- Implementers must satisfy more requirements.
- Supertrait chains can become hard to follow if overused.

## When Not To Use It

Avoid supertraits when the parent behavior is only sometimes needed. In that
case, a separate function with bounds may be more flexible.

## What To Notice In The Code

`trait PrettyPrint: Display` means every `PrettyPrint` type must also implement
`Display`. `Title` implements `Display`, then gets the default `pretty_print`
method through `PrettyPrint`.
