# nested_collections.rs

Source: `src/advanced_collections/nested_collections.rs`

## What This Code Is Used For

This code demonstrates nested collections with `HashMap<String, HashSet<String>>`.

## Why Nested Collections Are Chosen

The scenario stores many permissions for each user. A map gives lookup by user, and a set prevents duplicate permissions.

## Advantages

- Models real app relationships.
- Allows fast lookup by outer key.
- Inner sets prevent duplicates.

## Disadvantages

- Types can become hard to read.
- Deep nesting can make mutation verbose.

## When Not To Use It

Create named structs when the nested shape starts carrying business rules or many fields.

## What To Notice In The Code

The `PermissionsByUser` type alias makes the nested collection easier to read.
