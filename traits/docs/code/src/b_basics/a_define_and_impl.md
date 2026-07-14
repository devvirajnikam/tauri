# `src/a_basics/a_define_and_impl.rs`

## What This Code Is Used For

This example shows the core idea of traits: different structs can share the same
behavior name while implementing that behavior differently.

## Why This Pattern Is Chosen

`Article` and `Video` are different data types, but both can produce a summary.
The `Summary` trait captures the shared behavior without forcing both structs to
store the same fields.

## Advantages

- Code can depend on behavior instead of one exact struct.
- Each type keeps its own data shape.
- The method name stays consistent across types.

## Disadvantages

- A trait adds another concept to understand.
- If only one type ever needs the behavior, a normal method may be enough.

## When Not To Use It

Do not create a trait just because a method exists. Use a trait when multiple
types should share the same behavior contract or when you want generic code to
accept many types.

## What To Notice In The Code

`Summary` only defines what must exist: `summarize`. The `impl Summary for
Article` and `impl Summary for Video` blocks decide how that method works for
each type.
