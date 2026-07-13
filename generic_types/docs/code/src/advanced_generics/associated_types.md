# associated_types.rs

Source: `src/advanced_generics/associated_types.rs`

## What This Code Is Used For

This code demonstrates associated types.

## Why Associated Types Are Chosen

The trait has one main output type selected by each implementation. Associated types keep that relationship inside the impl.

## Advantages

- Cleaner than passing a type parameter everywhere.
- Common in iterator-like and loader-like traits.
- Each implementer chooses its output type once.

## Disadvantages

- Less flexible if one type needs multiple implementations for different output types.

## When Not To Use It

Use a generic trait when the same implementer should support many type choices.

## What To Notice In The Code

`UserLoader` sets `type Item = String`.
