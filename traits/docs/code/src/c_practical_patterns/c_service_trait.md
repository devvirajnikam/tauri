# `src/b_practical_patterns/c_service_trait.rs`

## What This Code Is Used For

This example shows how a service can depend on behavior instead of one concrete
dependency.

## Why This Pattern Is Chosen

`OtpService` needs to send SMS messages, but it does not need to know whether
SMS is sent through the console, a real provider, or a test fake.

## Advantages

- Services become easier to test.
- Low-level provider code can change without rewriting service logic.
- The dependency contract is small and explicit.

## Disadvantages

- Generic services can make type signatures longer.
- If there is only one dependency forever, the trait may not add much value.

## When Not To Use It

Avoid this pattern when the service and dependency are both tiny and unlikely to
change. Start concrete, then introduce a trait when a real need appears.

## What To Notice In The Code

`OtpService<T: SmsGateway>` stores any gateway that implements the trait. This
is dependency injection using Rust generics.
