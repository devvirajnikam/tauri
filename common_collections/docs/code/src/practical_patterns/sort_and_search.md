# sort_and_search.rs

Source: `src/practical_patterns/sort_and_search.rs`

## What This Code Is Used For

This code sorts a list and then searches it efficiently.

## Why Vec Sort And Binary Search Are Chosen

Sorting a Vec is good when you need ordered output. After sorting, `binary_search` can find a value faster than scanning linearly.

## Advantages

- Sorted output is easy to read and predictable.
- Binary search is efficient on sorted data.
- Works well for one-time report preparation.

## Disadvantages

- Sorting costs time upfront.
- `binary_search` only works correctly on sorted data.
- Inserting new values later may require maintaining sorted order.

## When Not To Use It

- Use `HashSet` for repeated membership checks without needing order.
- Use `BTreeSet` if values must stay sorted as they are inserted.

## What To Notice In The Code

The Vec is sorted before `binary_search`. The `match` handles both the found index and the insertion position if the value is missing.
