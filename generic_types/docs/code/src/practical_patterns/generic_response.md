# generic_response.rs

Source: `src/practical_patterns/generic_response.rs`

## What This Code Is Used For

This code demonstrates a generic API response wrapper.

## Why A Generic Wrapper Is Chosen

Many responses share the same outer structure but carry different data payloads.

## Advantages

- Keeps response shape consistent.
- Avoids duplicating wrapper structs.
- Lets each caller choose the payload type.

## Disadvantages

- Very generic response wrappers can hide domain meaning if overused.

## When Not To Use It

Use specific response structs when each response has unique fields and behavior.

## What To Notice In The Code

`ApiResponse<String>` and `ApiResponse<i32>` use the same struct with different data types.
