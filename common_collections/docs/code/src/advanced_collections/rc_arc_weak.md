# rc_arc_weak.rs

Source: `src/advanced_collections/rc_arc_weak.rs`

## What This Code Is Used For

This code demonstrates shared ownership with `Rc<T>`, `Arc<T>`, and `Weak<T>`.

## Why These Types Are Chosen

Use `Rc<T>` for single-threaded shared ownership. Use `Arc<T>` for thread-safe shared ownership. Use `Weak<T>` to reference shared data without keeping it alive.

## Advantages

- Allows multiple owners of the same value.
- Avoids copying large data.
- `Weak<T>` helps avoid ownership cycles.

## Disadvantages

- Adds reference-counting overhead.
- `Rc<T>` is not thread-safe.
- Shared ownership can make program ownership harder to reason about.

## When Not To Use It

Prefer normal ownership and borrowing when one clear owner exists.

## What To Notice In The Code

`Rc::clone` creates another owner. `Rc::downgrade` creates a weak reference. `Arc::clone` creates a thread-safe shared owner.
