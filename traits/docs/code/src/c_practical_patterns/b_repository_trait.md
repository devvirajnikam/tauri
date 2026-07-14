# `src/b_practical_patterns/b_repository_trait.rs`

## What This Code Is Used For

This example shows how a repository trait hides where data comes from.

## Why This Pattern Is Chosen

Business code often should not care whether users come from memory, a database,
an API, or a file. The `UserRepository` trait gives that business code one
behavior to depend on.

## Advantages

- Makes storage implementations replaceable.
- Makes testing easier because an in-memory repository can stand in for a real
  database.
- Keeps business code focused on the action it needs.

## Disadvantages

- Can add abstraction before it is needed.
- A repository trait may become too large if it tries to cover every query.

## When Not To Use It

If the code is a small script or has only one simple storage path, a concrete
repository type can be more direct.

## What To Notice In The Code

`print_user` accepts `&impl UserRepository`, so it is not tied to
`InMemoryUserRepository`. The in-memory type is only one possible implementation.
