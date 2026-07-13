# stderr_output.rs

Source: `src/stderr_output.rs`

## What This Code Is Used For

This code demonstrates writing normal output to stdout and diagnostic output to stderr.

## Why stderr Is Chosen

CLI programs often separate normal results from warnings, diagnostics, and errors.

## Advantages

- Shells can redirect stdout and stderr separately.
- Keeps normal output clean.
- Useful for logs and command-line tools.

## Disadvantages

- In some terminals both streams still appear together.
- Too much stderr output can feel noisy.

## When Not To Use It

Use stdout for normal expected program output.

## What To Notice In The Code

`println!` writes to stdout, while `eprintln!` writes to stderr.
