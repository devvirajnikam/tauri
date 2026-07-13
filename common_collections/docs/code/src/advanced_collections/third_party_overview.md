# third_party_overview.rs

Source: `src/advanced_collections/third_party_overview.rs`

## What This Code Is Used For

This code lists common third-party collection crates and what they are usually used for.

## Why It Is Included

The standard library covers many needs, but production code sometimes needs stable insertion order, small-list optimization, concurrent maps, or stable key storage.

## Advantages

- Helps you know what exists beyond std.
- Keeps this project dependency-free while still explaining the ecosystem.
- Gives names to search when a standard collection is not enough.

## Disadvantages

- The module only prints descriptions; it does not demonstrate the actual crates.
- Third-party crates add dependencies and version decisions.

## When Not To Use It

Do not add a third-party collection until a standard collection is clearly not enough.

## What To Notice In The Code

The examples mention `indexmap`, `smallvec`, `dashmap`, and `slab` as common next-step crates.
