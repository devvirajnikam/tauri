# parse_input.rs

Source: `src/parse_input.rs`

## What This Code Is Used For

This code reads text input and parses it into an `i32`.

## Why Parsing Is Separate From Reading

Terminal input arrives as text. Parsing is a second step because the user may type invalid data.

## Advantages

- Handles read errors and parse errors separately.
- Keeps the input flow realistic.
- Shows why parsing returns `Result`.

## Disadvantages

- More branches than reading plain text.
- The user may need another prompt in a real app if parsing fails.

## When Not To Use It

Do not parse directly with `unwrap` for user input unless the program is a quick throwaway demo.

## What To Notice In The Code

The code trims the input before parsing. Without trimming, the newline from Enter would make parsing fail.
