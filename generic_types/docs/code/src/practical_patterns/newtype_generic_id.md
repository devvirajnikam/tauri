# newtype_generic_id.rs

Source: `src/practical_patterns/newtype_generic_id.rs`

## What This Code Is Used For

This code demonstrates generic typed IDs.

## Why A Generic ID Is Chosen

`Id<User>` and `Id<Order>` can both store `u32`, but the type system treats them as different ID types.

## Advantages

- Prevents mixing IDs from different entities.
- Keeps runtime representation small.
- Documents intent in type signatures.

## Disadvantages

- Requires more type annotations.
- Uses `PhantomData`, which can feel abstract at first.

## When Not To Use It

Use plain IDs in tiny demos where type safety around entity identity is not important.

## What To Notice In The Code

`PhantomData<T>` marks which entity type the ID belongs to without storing a real `T`.
