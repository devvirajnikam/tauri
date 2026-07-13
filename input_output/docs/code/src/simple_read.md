# simple_read.rs

Source: `src/simple_read.rs`

## What This Code Is Used For

This code reads one line of text from the terminal.

## Why `read_line` Is Chosen

`read_line` is the basic standard-library way to read terminal input into a `String`.

## Advantages

- Simple to understand.
- Uses only the standard library.
- Good first step for command-line input.

## Disadvantages

- Input includes the trailing newline until trimmed.
- It reads text only; numbers need parsing afterward.
- `expect` can panic if reading fails.

## When Not To Use It

Avoid direct `expect` in user-facing app flows where input failure should be handled gracefully.

## What To Notice In The Code

The code creates a mutable `String`, passes it to `read_line`, then prints `trim_end()` to remove the newline from display.
