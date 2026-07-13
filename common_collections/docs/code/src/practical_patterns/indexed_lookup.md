# indexed_lookup.rs

Source: `src/practical_patterns/indexed_lookup.rs`

## What This Code Is Used For

This code converts a list of products into a lookup table by product ID.

## Why HashMap Is Chosen

Repeated lookup by ID is faster and clearer with `HashMap<u32, Product>` than scanning a Vec every time.

## Advantages

- Fast lookup by ID.
- Good for caching API results by primary key.
- Turns a list into an index that matches common app access patterns.

## Disadvantages

- Original list order is lost.
- Duplicate IDs would overwrite earlier entries.
- More memory is used than keeping only the Vec.

## When Not To Use It

- Keep a Vec if you only iterate once.
- Use `BTreeMap` if IDs need sorted iteration.

## What To Notice In The Code

`into_iter()` consumes the Vec, `map` converts each product into `(id, product)`, and `collect()` builds the HashMap.
