# `src/c_advanced_traits/d_where_clause_bounds.rs`

## What This Code Is Used For

This example shows how `where` clauses make trait bounds easier to read.

## Why This Pattern Is Chosen

When a type has multiple trait requirements, putting all bounds inside angle
brackets can make the function signature noisy. A `where` clause moves the
requirements into a readable block.

## Advantages

- Improves readability for complex generic functions.
- Keeps the function name and parameters easier to scan.
- Scales better when several types have several bounds.

## Disadvantages

- Slightly more syntax than a simple inline bound.
- Can feel unnecessary for one small bound.

## When Not To Use It

For simple cases like `fn print<T: Display>(value: T)`, inline bounds are often
shorter and clear enough.

## What To Notice In The Code

The function requires `T: Debug + Display`, so it can print the same value using
both `{}` and `{:?}`.
