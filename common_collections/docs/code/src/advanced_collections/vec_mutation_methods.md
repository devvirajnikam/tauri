# vec_mutation_methods.rs

Source: `src/advanced_collections/vec_mutation_methods.rs`

## What This Code Is Used For

This code demonstrates `retain`, `drain`, and `split_off` on Vec.

## Why These Methods Are Chosen

They express common Vec mutations directly without fragile manual index management.

## Advantages

- `retain` keeps only matching items.
- `drain` removes and returns a range.
- `split_off` moves the tail into a new Vec.

## Disadvantages

- These methods mutate the original Vec.
- Ranges must be valid or the code can panic.

## When Not To Use It

Use iterator transformations when you want a new Vec and want to keep the original unchanged.

## What To Notice In The Code

The example keeps even numbers, drains part of the Vec, and splits a second Vec into head and tail.
