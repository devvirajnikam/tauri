# command_loop.rs

Source: `src/command_loop.rs`

## What This Code Is Used For

This code demonstrates a small command loop.

## Why A Loop And `match` Are Chosen

A command-line program often reads commands repeatedly. `match` cleanly maps each command string to behavior.

## Advantages

- Good foundation for CLI menus.
- Handles known commands clearly.
- Has an exit path with `break`.

## Disadvantages

- String matching can become messy as commands grow.
- Real CLIs often need argument parsing, validation, and command structs.

## When Not To Use It

Use a CLI parser crate for production command-line tools with flags, subcommands, or complex arguments.

## What To Notice In The Code

The loop keeps reading until the user enters `exit` or an IO error occurs.
