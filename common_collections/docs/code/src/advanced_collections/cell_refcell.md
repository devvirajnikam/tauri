# cell_refcell.rs

Source: `src/advanced_collections/cell_refcell.rs`

## What This Code Is Used For

This code demonstrates `Cell<T>` and `RefCell<T>`.

## Why These Types Are Chosen

Use them when a value needs interior mutability, meaning it can change through a shared reference.

## Advantages

- `Cell<T>` is simple for Copy values.
- `RefCell<T>` allows mutable access checked at runtime.
- Useful with `Rc<T>` in single-threaded shared structures.

## Disadvantages

- `RefCell<T>` can panic at runtime if borrowing rules are violated.
- Not thread-safe.
- Can hide mutation and make code harder to follow.

## When Not To Use It

Use normal `&mut` borrowing when possible. Use `Mutex<T>` or `RwLock<T>` for thread-safe interior mutability.

## What To Notice In The Code

`Cell` updates a number using `get` and `set`. `RefCell` mutates a Vec through `borrow_mut()`.
