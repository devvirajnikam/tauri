# arrays_capacity.rs

Source: `src/advanced_collections/arrays_capacity.rs`

## What This Code Is Used For

This code demonstrates fixed arrays and Vec capacity.

## Why These Concepts Are Chosen

Arrays are useful when size is fixed. `Vec::with_capacity` is useful when you know roughly how many values will be pushed.

## Advantages

- Arrays avoid growth logic.
- Preallocating Vec capacity can reduce reallocations.
- Capacity makes performance intent clearer.

## Disadvantages

- Arrays cannot grow.
- Overestimating capacity can waste memory.
- Capacity is an optimization detail, not usually needed first.

## When Not To Use It

Use plain `Vec::new()` until capacity is known to matter.

## What To Notice In The Code

The array has a fixed size. The Vec starts with reserved capacity for three items.
