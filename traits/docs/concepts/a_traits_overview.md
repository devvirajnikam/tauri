# Traits Overview

## What Is A Trait?

A trait is a contract for behavior.

It says: any type that implements this trait must provide these methods.

For example, if a trait says `summarize`, then every type implementing that
trait must know how to summarize itself. An article can summarize itself using a
title and author. A video can summarize itself using a title and duration. The
behavior name is the same, but the implementation can be different.

## Why Traits Exist

Rust does not use classical inheritance as the main way to share behavior.
Instead, Rust uses composition, structs, enums, generics, and traits.

Traits solve this problem:

Different types may not share the same data, but they can still share the same
behavior.

That lets you write code based on what a value can do, not only what exact type
it is.

## Simple Mental Model

Think of a trait like a promise.

If a type implements a trait, it promises that certain methods are available.
After that, other code can safely call those methods.

Rust checks this promise at compile time. If a type does not implement the
required behavior, the program will not compile.

## Why Traits Are Important In Rust

Traits are used everywhere in Rust.

Common examples from the standard library include:

- `Debug`: allows a value to be printed with `{:?}`
- `Clone`: allows a value to create a duplicate of itself
- `Default`: allows a type to provide a default value
- `PartialEq`: allows values to be compared with `==`
- `Iterator`: allows repeated access to a sequence of values
- `From` and `Into`: convert one type into another type
- `Display`: allows user-friendly formatting with `{}`

When you write Rust regularly, you are constantly using traits even when you are
not creating your own.

## Trait Compared With JavaScript

JavaScript does not have Rust-style traits built into the language.

In JavaScript, behavior is usually shared through objects, prototypes, classes,
duck typing, or manually following a convention.

Example JavaScript idea:

```javascript
const article = {
  title: "Traits in Rust",
  author: "Asha",
  summarize() {
    return `Article '${this.title}' by ${this.author}`;
  },
};

const video = {
  title: "Rust trait basics",
  durationMinutes: 12,
  summarize() {
    return `Video '${this.title}' - ${this.durationMinutes} minutes`;
  },
};

function printSummary(item) {
  console.log(item.summarize());
}

printSummary(article);
printSummary(video);
```

This works because JavaScript checks at runtime whether `summarize` exists.

If you pass an object without `summarize`, the program fails while running.

Rust moves that safety check earlier.

Rust version of the same idea:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

In Rust, `print_summary` cannot receive a type unless that type implements
`Summary`. The compiler checks this before the program runs.

## JavaScript Duck Typing Vs Rust Traits

JavaScript often uses duck typing:

If an object has the method you call, it works.

Rust traits are explicit:

The type must declare that it implements the trait.

This explicitness gives Rust stronger safety and better tooling. The compiler
knows exactly what behavior is required.

## Traits Compared With JavaScript Classes

JavaScript classes can share behavior through inheritance:

```javascript
class Notifier {
  notify(message) {
    console.log(message);
  }
}

class EmailNotifier extends Notifier {}
```

Rust traits are different from class inheritance.

A trait does not store fields. It describes behavior. The struct stores data,
and the trait implementation describes how that struct performs the behavior.

This means Rust avoids many inheritance problems, such as deep parent-child
chains and unclear ownership of behavior.

## Traits Compared With TypeScript Interfaces

Traits are closer to TypeScript interfaces than plain JavaScript classes.

TypeScript example:

```typescript
interface Summary {
  summarize(): string;
}
```

Rust trait:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

The big difference is that Rust traits can also provide default method
implementations, participate in generics, support associated types, and be used
for operator behavior.

## Why Not Just Use Struct Methods?

Struct methods are enough when only one type needs the behavior.

Use a normal method when the behavior belongs to one concrete type:

```rust
impl Invoice {
    fn total(&self) -> u32 {
        self.amount
    }
}
```

Use a trait when multiple types should share the same behavior contract:

```rust
trait Payable {
    fn total(&self) -> u32;
}
```

The trait becomes useful when `Invoice`, `PurchaseOrder`, and `Subscription` all
need to be handled through the same behavior.

## Why Not Just Use Enums?

Enums are good when you know all possible variants in advance.

For example:

```rust
enum Notification {
    Email,
    Sms,
}
```

This is useful when the set is closed and controlled by your code.

Traits are better when the set of possible implementations should stay open.
For example, today you may have `EmailNotifier` and `SmsNotifier`. Tomorrow
another module may add `SlackNotifier` without changing the original enum.

Use an enum when the possibilities are fixed.

Use a trait when new implementations should be able to appear independently.

## Static Dispatch And Dynamic Dispatch

Rust traits can be used in two important ways.

Static dispatch uses generics or `impl Trait`:

```rust
fn send(notifier: &impl Notifier) {
    notifier.notify("hello");
}
```

Rust knows the concrete type at compile time. This is usually fast and strongly
optimized.

Dynamic dispatch uses `dyn Trait`:

```rust
let items: Vec<Box<dyn Draw>> = vec![];
```

Rust chooses the method at runtime. This is useful when different concrete types
must be stored together in one collection.

## When Traits Are A Good Choice

Traits are a good choice when:

- Multiple types share the same behavior
- You want generic functions that work with many types
- You want to swap implementations, such as real service vs test service
- You want business code to depend on behavior instead of storage details
- You want to model a reusable pattern like validation, formatting, or strategy

## When Traits Are Not Needed

Traits may be unnecessary when:

- Only one concrete type needs the behavior
- A simple function is enough
- An enum clearly models all possible cases
- The abstraction makes the code harder instead of easier

Good Rust code does not create traits everywhere. It creates traits where shared
behavior or replaceable implementation is actually useful.

## How This Project Is Organized

The project is split into three learning levels.

`basics` explains the core syntax:

- define a trait
- implement a trait
- use default methods
- use trait bounds
- use `impl Trait`

`practical_patterns` shows real-world usage:

- validators
- repositories
- service dependencies
- strategy pattern
- formatters

`advanced_traits` explains deeper trait features:

- trait objects
- associated types
- supertraits
- `where` clauses
- operator traits
- derived traits

## Main Takeaway

Traits let Rust code say:

I do not care exactly what type this is. I care what behavior it provides.

That is why traits are one of the most important concepts in Rust.
