# btree_set.rs

Source: `src/common/btree_set.rs`

## What This Code Is Used For

This code demonstrates `BTreeSet<T>`, a sorted unique-value collection.

## Why BTreeSet Is Chosen

The scenario needs invoice numbers to be unique and sorted. A `HashSet` would remove duplicates but would not provide sorted order.

## Advantages

- Removes duplicates.
- Keeps values sorted.
- Deterministic output.

## Disadvantages

- Usually slower than HashSet for membership checks.
- Values must implement `Ord`.

## When Not To Use It

- Use `HashSet` when sorted order is not needed.
- Use `Vec` when duplicates are meaningful.

## What To Notice In The Code

The duplicate invoice number is ignored, and the printed set is sorted.
