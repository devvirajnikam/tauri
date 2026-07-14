# `src/b_practical_patterns/e_formatter_trait.rs`

## What This Code Is Used For

This example shows how traits can separate data from presentation.

## Why This Pattern Is Chosen

The same `Report` can be shown as plain text, JSON-like text, HTML, CSV, or any
other output format. A formatter trait keeps formatting decisions outside the
data struct.

## Advantages

- Adds new output formats without changing the report data type.
- Keeps formatting logic isolated.
- Makes formatting implementations easy to test independently.

## Disadvantages

- Can be unnecessary if only one format exists.
- Real JSON should use a proper serializer instead of manual string building.

## When Not To Use It

Do not hand-build structured formats like JSON in production when libraries such
as `serde_json` are available. This example uses a simple string only to teach
the trait pattern.

## What To Notice In The Code

`PlainTextFormatter` and `JsonLikeFormatter` both implement `ReportFormatter`,
so the same report can be presented in different ways.
