# recovery_strategies.rs

Source: `src/advanced_errors/recovery_strategies.rs`

## What This Code Is Used For

This code demonstrates choosing different recovery behavior for different errors.

## Why Recovery Strategy Is Chosen

Some errors should retry, some should use fallback data, and some should stop. Encoding that in an enum makes the decision explicit.

## Advantages

- Clear operational behavior.
- Easy to match temporary vs permanent failures.
- Avoids treating all errors the same.

## Disadvantages

- Requires careful error classification.
- Wrong classification can cause bad retries or hidden failures.

## When Not To Use It

Do not add retry/fallback logic unless the system can actually recover safely.

## What To Notice In The Code

`FetchError::Temporary` maps to retry behavior, while `FetchError::Permanent` maps to fallback behavior.
