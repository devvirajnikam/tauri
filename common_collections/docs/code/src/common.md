# common.rs

Source: `src/common.rs`

## What This Code Is Used For

This module groups the standard collection examples: `Vec`, `String`, `HashMap`, `HashSet`, `VecDeque`, `BTreeMap`, `BTreeSet`, `BinaryHeap`, `LinkedList`, arrays, and slices.

## Why This Structure Is Better

Each collection has its own module, but `common.rs` gives one clear entry point for running them all.

## Advantages

- Easy to scan the list of covered collections.
- Each topic remains isolated.
- The run order is controlled from one place.

## Disadvantages

- Adding a new collection requires updating this file and creating a new module file.

## What To Notice

The `run()` function calls each collection example in a learning order, starting with the most common choices.
