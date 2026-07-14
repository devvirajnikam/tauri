# `src/c_advanced_traits/a_trait_objects.rs`

## What This Code Is Used For

This example shows how to store different concrete types in one collection when
they share a trait.

## Why This Pattern Is Chosen

`Button` and `TextInput` are different structs, but a UI screen may want to keep
them in the same list and call `draw` on each one. `Box<dyn Draw>` enables that.

## Advantages

- Allows mixed concrete types in one collection.
- Supports runtime polymorphism.
- Useful for plugins, UI components, commands, and handlers.

## Disadvantages

- Uses dynamic dispatch, so Rust chooses the method at runtime.
- Usually needs a pointer type such as `Box`, `Rc`, or `Arc`.
- Some trait features are restricted for trait objects.

## When Not To Use It

Use generics when the concrete type is known and does not need to be mixed with
other types in the same collection.

## What To Notice In The Code

`Vec<Box<dyn Draw>>` stores boxed trait objects. The vector does not know whether
each item is a `Button` or `TextInput`; it only knows each item can `draw`.
