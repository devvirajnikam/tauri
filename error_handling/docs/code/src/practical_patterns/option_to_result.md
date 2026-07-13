# option_to_result.rs

Source: `src/practical_patterns/option_to_result.rs`

## What This Code Is Used For

This code converts a missing optional value into a real error.

## Why This Pattern Is Chosen

`Option` only says data is missing. `Result` can explain why missing data is a failure.

## Advantages

- Adds useful error context.
- Keeps lookup functions simple.
- `ok_or_else` avoids building the error unless needed.

## Disadvantages

- Adds more error handling code.
- The chosen error type still matters.

## When Not To Use It

Keep `Option` when missing data is normal and does not need explanation.

## What To Notice In The Code

`find_user_name` returns `Option`, while `require_user_name` upgrades missing data into `Result`.
