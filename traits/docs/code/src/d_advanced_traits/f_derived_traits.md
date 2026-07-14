# `src/c_advanced_traits/f_derived_traits.rs`

## What This Code Is Used For

This example shows compiler-generated implementations for common traits.

## Why This Pattern Is Chosen

Types frequently need debug printing, cloning, equality checks, or default
values. `derive` asks the compiler to generate those implementations instead of
writing repetitive boilerplate by hand.

## Advantages

- Saves repetitive code.
- Reduces mistakes in common trait implementations.
- Makes a type easier to inspect, clone, compare, and initialize.

## Disadvantages

- Derived behavior is generic and may not match custom business rules.
- Some traits can only be derived when all fields support the same trait.

## When Not To Use It

Write the implementation manually when the behavior needs custom logic, such as
case-insensitive equality or a default value that depends on application rules.

## What To Notice In The Code

`Debug` enables `{:?}`, `Clone` enables `.clone()`, `PartialEq` enables `==`, and
`Default` enables `Settings::default()`.
