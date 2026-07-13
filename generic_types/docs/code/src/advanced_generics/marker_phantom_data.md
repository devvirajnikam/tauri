# marker_phantom_data.rs

Source: `src/advanced_generics/marker_phantom_data.rs`

## What This Code Is Used For

This code demonstrates marker-state generics with `PhantomData`.

## Why `PhantomData` Is Chosen

The post state matters to the type system, but no real state object needs to be stored at runtime.

## Advantages

- Enforces state transitions at compile time.
- Prevents calling published-only methods on draft values.
- Adds type safety without extra runtime data.

## Disadvantages

- More abstract than a simple enum field.
- Can be overkill for simple apps.

## When Not To Use It

Use a normal enum state field when runtime flexibility is more important than compile-time state restrictions.

## What To Notice In The Code

Only `Post<Published>` has the `show` method.
