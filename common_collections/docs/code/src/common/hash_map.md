# hash_map.rs

Source: `src/common/hash_map.rs`

## What This Code Is Used For

This code demonstrates `HashMap<K, V>`, a key-value collection used for fast lookup.

## Why HashMap Is Chosen

The scenario maps item names to stock quantities. A map is better than a Vec here because the code wants to find stock by item name, not by position.

## Advantages

- Fast average-case lookup by key.
- Good for indexes, counters, caches, and lookup tables.
- The `entry` API makes update-or-insert logic clean.

## Disadvantages

- Does not keep keys sorted.
- Iteration order is not stable.
- Keys must implement `Hash` and `Eq`.

## When Not To Use It

- Use `BTreeMap` when sorted key order matters.
- Use `Vec` when the data is small and order matters more than lookup speed.

## What To Notice In The Code

The example uses `insert` for key-value pairs, `get` for safe lookup, and `entry(...).or_insert(0)` to update a value even when the key may not exist yet.
