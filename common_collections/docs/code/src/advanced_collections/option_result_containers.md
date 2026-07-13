# option_result_containers.rs

Source: `src/advanced_collections/option_result_containers.rs`

## What This Code Is Used For

This code treats `Option<T>` and `Result<T, E>` as small container-like enums.

## Why They Are Included

They are not collections like Vec, but they hold values in structured ways: zero-or-one value for `Option`, success-or-error for `Result`.

## Advantages

- Forces missing/error cases to be handled.
- Works well with helpers like `map`, `unwrap_or`, and `?`.
- Makes APIs safer than nulls or unchecked errors.

## Disadvantages

- Adds explicit handling code.
- Beginners may find nested `Option<Result<T, E>>` shapes confusing.

## When Not To Use It

Do not use `Option` for real errors that need explanation. Use `Result` for those.

## What To Notice In The Code

`find_user_name` returns `Option<String>`. `parse_quantity` returns `Result<u32, String>`.
