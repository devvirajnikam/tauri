# `src/b_practical_patterns/d_strategy_trait.rs`

## What This Code Is Used For

This example shows the strategy pattern using traits.

## Why This Pattern Is Chosen

Discount calculation can change depending on a business rule. A trait lets each
discount algorithm live in its own type while the calling code uses the same
method.

## Advantages

- Makes algorithms interchangeable.
- Keeps each strategy small and focused.
- Avoids large `match` or `if` chains when behavior belongs in separate types.

## Disadvantages

- Too many tiny strategy types can feel heavy.
- If the rules are simple data choices, an enum may be easier.

## When Not To Use It

Use an enum when the set of strategies is small, fixed, and all logic belongs in
one module. Use traits when implementations should be open-ended or replaceable.

## What To Notice In The Code

`NoDiscount` and `TenPercentDiscount` are separate types with the same
`discount` behavior. `final_amount` only depends on the trait.
