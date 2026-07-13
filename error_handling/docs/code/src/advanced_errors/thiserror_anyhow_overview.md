# thiserror_anyhow_overview.rs

Source: `src/advanced_errors/thiserror_anyhow_overview.rs`

## What This Code Is Used For

This code gives a dependency-free overview of common error crates.

## Why It Is Included

Manual error implementations are useful to learn, but production Rust often uses crates to reduce boilerplate.

## Advantages

- `thiserror` is good for custom error enums.
- `anyhow` is good for application-level error reporting.
- `eyre` is another app-level reporting option.

## Disadvantages

- Adds dependencies.
- Crate choice depends on whether you are writing a library or an application.

## When Not To Use It

Do not add a crate before understanding what problem it solves.

## What To Notice In The Code

The project only prints the overview, so it stays dependency-free and easy to run.
