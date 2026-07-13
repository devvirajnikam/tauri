# generic_methods.rs

Source: `src/advanced_generics/generic_methods.rs`

## What This Code Is Used For

This code demonstrates generic methods.

## Why A Generic Method Is Chosen

The `Logger` type itself is not generic, but its `log` method can accept many displayable value types.

## Advantages

- Keeps the struct simple.
- Makes only the method generic.
- Useful for utility methods.

## Disadvantages

- Each generic method call is still checked against its bounds.

## When Not To Use It

Make the whole struct generic only when it stores or owns generic data.

## What To Notice In The Code

`Logger` can log both a string and a number because the method has `T: Display`.
