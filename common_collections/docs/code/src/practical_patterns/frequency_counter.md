# frequency_counter.rs

Source: `src/practical_patterns/frequency_counter.rs`

## What This Code Is Used For

This code counts how often each word appears.

## Why HashMap Is Chosen

A frequency counter needs one count per unique key. `HashMap<&str, usize>` is a natural shape for that.

## Advantages

- Efficiently updates counts by key.
- The `entry` API avoids separate check-then-insert code.
- Works for words, statuses, IDs, categories, and many similar cases.

## Disadvantages

- Output order is not stable.
- The key type must be hashable.

## When Not To Use It

- Use `BTreeMap` if you need counts printed in sorted key order.

## What To Notice In The Code

`entry(word).or_insert(0)` gets the existing count or creates a zero count, then `*count += 1` updates it in place.
