# generic_validator.rs

Source: `src/practical_patterns/generic_validator.rs`

## What This Code Is Used For

This code demonstrates a generic validation trait.

## Why A Generic Trait Is Chosen

Different validation rules work on different input types, but they can share the same `validate` method shape.

## Advantages

- One trait describes many validation rules.
- Each implementation chooses its own input type.
- Useful for reusable validation systems.

## Disadvantages

- Generic traits can become abstract quickly.
- A simple function may be enough for one rule.

## When Not To Use It

Avoid a trait if validation is local and not reused.

## What To Notice In The Code

`NonEmpty` validates `String`, while `Minimum` validates `i32`.
