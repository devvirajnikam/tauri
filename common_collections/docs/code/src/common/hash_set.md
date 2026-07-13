# hash_set.rs

Source: `src/common/hash_set.rs`

## What This Code Is Used For

This code demonstrates `HashSet<T>`, a collection for unique values.

## Why HashSet Is Chosen

The scenario stores selected IDs. A set is better than a Vec because duplicate selected IDs should not exist.

## Advantages

- Automatically prevents duplicates.
- Fast average-case membership checks.
- Useful for selected IDs, tags, visited items, and permission sets.

## Disadvantages

- Does not preserve insertion order.
- Does not sort values.
- Values must implement `Hash` and `Eq`.

## When Not To Use It

- Use `BTreeSet` if you need sorted unique values.
- Use `Vec` if duplicates and order are both meaningful.

## What To Notice In The Code

The duplicate `insert(2)` does not create a second `2`. `contains` checks membership, and `remove` deletes a value.
