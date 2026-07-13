# remove_duplicates.rs

Source: `src/practical_patterns/remove_duplicates.rs`

## What This Code Is Used For

This code removes duplicate IDs from a list.

## Why HashSet Is Chosen

A set stores each value only once, so collecting into a `HashSet` is a direct way to remove duplicates.

## Advantages

- Very simple deduplication.
- Fast for large lists.
- Good for IDs, tags, and unique selections.

## Disadvantages

- HashSet does not preserve original order.
- The result is not sorted unless you sort it after converting back to Vec.

## When Not To Use It

- Use `Vec::dedup` if the Vec is already sorted and you want to stay in Vec form.
- Use `BTreeSet` if you want uniqueness and sorted order together.

## What To Notice In The Code

The list is consumed into a `HashSet`, converted back into a Vec, and sorted for stable display.
