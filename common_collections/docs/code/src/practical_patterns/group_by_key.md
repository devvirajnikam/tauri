# group_by_key.rs

Source: `src/practical_patterns/group_by_key.rs`

## What This Code Is Used For

This code groups employees by department.

## Why HashMap<Key, Vec<Value>> Is Chosen

Grouping means one key can have many values. A map from department to a Vec of employee names is the standard shape.

## Advantages

- Efficiently groups records in one pass.
- Handles new groups automatically with `or_default`.
- Easy to extend to reports and categorized UI lists.

## Disadvantages

- Group order is not stable with HashMap.
- Values are moved into the grouped structure, so the original records are consumed.

## When Not To Use It

- Use `BTreeMap` if departments should be sorted.
- Keep a Vec if you only need to filter once and do not need grouped lookup.

## What To Notice In The Code

The code consumes each `Employee`, uses the department as the key, and pushes the name into that department's Vec.
