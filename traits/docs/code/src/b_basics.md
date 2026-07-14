# `src/a_basics.rs`

## What This Code Is Used For

This file groups the beginner-level trait examples.

## Why This Pattern Is Chosen

Traits can feel abstract at first, so the project starts with simple building
blocks before moving into real-world design patterns.

## Advantages

- Gives a clear starting point for trait learning.
- Keeps beginner concepts separate from advanced syntax.
- Runs all basic examples from one place.

## Disadvantages

- This file only organizes modules; the real examples are in child files.
- It does not explain each concept by itself.

## When Not To Use It

If a project has only one or two examples, a grouping file may be unnecessary.

## What To Notice In The Code

The order starts with defining a trait, then default methods, trait bounds, and
finally `impl Trait` parameters. That order follows how trait knowledge usually
builds up.
