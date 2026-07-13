# btree_map.rs

Source: `src/common/btree_map.rs`

## What This Code Is Used For

This code demonstrates `BTreeMap<K, V>`, a sorted key-value map.

## Why BTreeMap Is Chosen

The scenario prints sales by month in key order. `BTreeMap` is better than `HashMap` when sorted iteration matters.

## Advantages

- Keeps keys sorted.
- Iteration order is deterministic.
- Useful for reports, ranges, and ordered output.

## Disadvantages

- Usually slower than HashMap for simple lookup.
- Keys must implement `Ord`.

## When Not To Use It

- Use `HashMap` when sorted order does not matter and fast lookup is the priority.

## What To Notice In The Code

The months are inserted out of order, but iteration prints them in sorted key order.
