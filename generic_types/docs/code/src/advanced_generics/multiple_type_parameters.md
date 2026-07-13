# multiple_type_parameters.rs

Source: `src/advanced_generics/multiple_type_parameters.rs`

## What This Code Is Used For

This code demonstrates a generic type with two type parameters.

## Why Multiple Type Parameters Are Chosen

The key and value may have different types, so `KeyValue<K, V>` lets each field choose independently.

## Advantages

- Flexible field types.
- Useful for pairs, maps, responses, and conversion types.

## Disadvantages

- More type parameters can make signatures harder to read.

## When Not To Use It

Use one type parameter when all fields should have the same type.

## What To Notice In The Code

`K` becomes `&str`, while `V` becomes `String`.
