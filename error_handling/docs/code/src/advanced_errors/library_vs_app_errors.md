# library_vs_app_errors.rs

Source: `src/advanced_errors/library_vs_app_errors.rs`

## What This Code Is Used For

This code compares library-style and application-style errors.

## Why This Pattern Is Chosen

Libraries should expose specific errors so callers can react. Applications often convert errors into messages for logging or display.

## Advantages

- Encourages stable library error APIs.
- Lets app code simplify final reporting.
- Separates reusable domain errors from presentation concerns.

## Disadvantages

- Requires deciding where each error type belongs.
- Too much conversion can lose useful structure.

## When Not To Use It

Do not convert structured errors to strings too early if later code needs to match them.

## What To Notice In The Code

`library_function` returns `LibraryError`, while `application_flow` converts it into a user-facing `String`.
