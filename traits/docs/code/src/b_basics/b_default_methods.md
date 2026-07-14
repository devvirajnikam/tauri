# `src/a_basics/b_default_methods.rs`

## What This Code Is Used For

This example shows how a trait can provide reusable behavior through a default
method.

## Why This Pattern Is Chosen

Most types that can greet someone only need to provide a name. The trait can use
that required `name` method to build the default `greet` behavior.

## Advantages

- Reduces repeated code across implementations.
- Allows each type to override the default if needed.
- Keeps common behavior close to the trait contract.

## Disadvantages

- Too much default behavior can hide important logic inside the trait.
- A default method can become hard to change if many types rely on it.

## When Not To Use It

Avoid default methods when each implementation should behave very differently.
In that case, requiring every type to implement the method directly is clearer.

## What To Notice In The Code

`Greet` requires `name`, but provides `greet`. `User` only implements `name`, and
then automatically gets the default greeting behavior.
