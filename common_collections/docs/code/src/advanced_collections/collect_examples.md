# collect_examples.rs

Source: `src/advanced_collections/collect_examples.rs`

## What This Code Is Used For

This code demonstrates collecting iterator output into different collection types.

## Why `collect` Is Chosen

`collect()` is the standard way to turn an iterator into a concrete collection.

## Advantages

- One iterator pipeline can become many target types.
- Works with Vec, HashSet, HashMap, String, and custom FromIterator types.
- Keeps transformation code compact.

## Disadvantages

- Rust sometimes needs type hints.
- `collect()` can allocate memory.

## When Not To Use It

Use a loop if collecting hides important branching or error handling.

## What To Notice In The Code

The same basic iterator idea becomes a Vec, HashSet, HashMap, and String based on the target type.
