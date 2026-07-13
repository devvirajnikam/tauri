# partition_examples.rs

Source: `src/advanced_collections/partition_examples.rs`

## What This Code Is Used For

This code demonstrates `partition()`.

## Why `partition` Is Chosen

Use it when every item should go into one of two groups based on a condition.

## Advantages

- Clear two-way split.
- Avoids writing two push loops manually.
- Works with iterator pipelines.

## Disadvantages

- Only splits into two groups.
- Consumes the iterator.

## When Not To Use It

Use `HashMap<Key, Vec<Value>>` for more than two groups.

## What To Notice In The Code

The numbers are split into even and odd Vec values in one expression.
