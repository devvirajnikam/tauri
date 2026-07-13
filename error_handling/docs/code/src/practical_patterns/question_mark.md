# question_mark.rs

Source: `src/practical_patterns/question_mark.rs`

## What This Code Is Used For

This code demonstrates the `?` operator.

## Why `?` Is Chosen

It keeps multi-step error handling readable by returning early on `Err`.

## Advantages

- Reduces repetitive `match` code.
- Keeps the success path easy to read.
- Preserves the error value.

## Disadvantages

- Can hide early returns from beginners.
- Requires compatible error types or conversions.

## When Not To Use It

Use explicit `match` when each error needs different recovery behavior.

## What To Notice In The Code

`calculate_total` stops immediately if `parse_quantity` returns an error.
