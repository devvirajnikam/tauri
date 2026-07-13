# generic_repository.rs

Source: `src/practical_patterns/generic_repository.rs`

## What This Code Is Used For

This code demonstrates a generic repository trait.

## Why A Generic Repository Is Chosen

The storage behavior can be described once, while each implementation chooses the entity type.

## Advantages

- Reuses repository behavior shape.
- Useful for app architecture examples.
- Makes the entity type explicit.

## Disadvantages

- Real repositories often need error types, filters, pagination, and async behavior.
- Too much generic repository abstraction can become awkward.

## When Not To Use It

Use concrete repository methods when each entity has very different query behavior.

## What To Notice In The Code

`Repository<User>` means this repository returns `User` values.
