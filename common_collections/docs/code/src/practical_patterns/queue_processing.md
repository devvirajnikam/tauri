# queue_processing.rs

Source: `src/practical_patterns/queue_processing.rs`

## What This Code Is Used For

This code processes jobs in first-in-first-out order.

## Why VecDeque Is Chosen

A queue needs efficient removal from the front. `VecDeque` is better than Vec for that because Vec front removal shifts all later items.

## Advantages

- Natural queue behavior.
- Efficient front removal.
- Clear `while let Some(job) = pop_front()` loop.

## Disadvantages

- Not needed for simple lists.
- Does not provide priority behavior.

## When Not To Use It

- Use `BinaryHeap` if the most important job should run first.
- Use `Vec` if you only add and remove from the end.

## What To Notice In The Code

`VecDeque::from([...])` creates the queue, and `pop_front` consumes jobs until the queue is empty.
