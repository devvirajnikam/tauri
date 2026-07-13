# error_source_chain.rs

Source: `src/advanced_errors/error_source_chain.rs`

## What This Code Is Used For

This code demonstrates error source chaining.

## Why Source Chains Are Chosen

High-level errors should explain the app context while preserving the lower-level cause for debugging.

## Advantages

- Keeps original error details.
- Makes debugging easier.
- Supports layered error design.

## Disadvantages

- More boilerplate with manual `Error` implementation.
- Can be overkill for simple demos.

## When Not To Use It

Use simpler errors when there is no meaningful lower-level source.

## What To Notice In The Code

`ConfigError` displays the high-level field problem and exposes the parse error through `source()`.
