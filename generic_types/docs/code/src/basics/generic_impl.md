# generic_impl.rs

Source: `src/basics/generic_impl.rs`

## What This Code Is Used For

This code demonstrates methods on a generic struct.

## Why `impl<T>` Is Chosen

`Pair<T>` should have `new` and `into_tuple` methods for every possible `T`.

## Advantages

- Methods are available for all concrete versions of the type.
- Keeps construction and conversion behavior close to the struct.

## Disadvantages

- Methods inside plain `impl<T>` cannot use behavior that requires a trait bound unless the bound is added.

## When Not To Use It

Use a bounded impl like `impl<T: Display>` when methods need special behavior from `T`.

## What To Notice In The Code

`into_tuple(self)` consumes the pair and moves both values out.
