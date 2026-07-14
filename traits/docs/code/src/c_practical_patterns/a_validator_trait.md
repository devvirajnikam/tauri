# `src/b_practical_patterns/a_validator_trait.rs`

## What This Code Is Used For

This example shows how validation rules can be represented as reusable traits.

## Why This Pattern Is Chosen

Different fields can have different validation rules, but the application can
call all of them through the same `validate` method.

## Advantages

- Makes validation rules easy to swap and combine.
- Keeps validation logic separate from business flow.
- The return type clearly communicates success or failure.

## Disadvantages

- A trait can be unnecessary for one simple validation check.
- Generic validator traits can become complex when validation needs context.

## When Not To Use It

For one-off validation inside a small function, a simple `if` check may be
clearer.

## What To Notice In The Code

`Validator<T>` is generic, so the same trait idea can validate different value
types. `NonEmpty` implements it specifically for `String`.
