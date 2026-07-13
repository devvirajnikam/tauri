# binary_heap.rs

Source: `src/common/binary_heap.rs`

## What This Code Is Used For

This code demonstrates `BinaryHeap<T>`, Rust's priority queue collection.

## Why BinaryHeap Is Chosen

The scenario processes the highest priority first. `BinaryHeap` is better than sorting a Vec repeatedly because it is designed for repeated priority pops.

## Advantages

- Efficiently returns the largest item first.
- Good for schedulers, priority queues, and pathfinding algorithms.
- Avoids sorting the whole collection after every insert.

## Disadvantages

- Does not maintain full sorted order internally.
- Default behavior is max-heap, so min-heap needs `Reverse` or custom ordering.
- Not ideal if you need to iterate all values in sorted order directly.

## When Not To Use It

- Use `Vec` plus `sort` for one-time sorting.
- Use `VecDeque` for first-in-first-out queues.

## What To Notice In The Code

Values are pushed in mixed order, but `pop` returns them from highest to lowest.
