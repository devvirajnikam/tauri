# vec_deque.rs

Source: `src/common/vec_deque.rs`

## What This Code Is Used For

This code demonstrates `VecDeque<T>`, a double-ended queue.

## Why VecDeque Is Chosen

The scenario needs queue behavior: add jobs and process them from the front. `VecDeque` handles front and back operations efficiently.

## Advantages

- Efficient `push_front`, `push_back`, `pop_front`, and `pop_back`.
- Better than `Vec` for queue behavior.
- Keeps logical order of processing.

## Disadvantages

- Slightly more complex internally than Vec.
- Not usually needed if you only push/pop at the end.

## When Not To Use It

- Use `Vec` for a simple growable list.
- Use `BinaryHeap` when priority matters more than insertion order.

## What To Notice In The Code

The example uses `push_back` for normal jobs, `push_front` for an urgent job, and `while let Some(...) = pop_front()` to process until the queue is empty.
