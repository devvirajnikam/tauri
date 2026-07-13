// Scenario:
// A type should carry compile-time state without storing real state data.
//
// Thinking:
// PhantomData<T> marks that a generic type parameter matters to the type system
// even when no field directly stores T.

use std::marker::PhantomData;

#[derive(Debug)]
struct Draft;

#[derive(Debug)]
struct Published;

#[derive(Debug)]
struct Post<State> {
    title: String,
    state: PhantomData<State>,
}

impl Post<Draft> {
    fn new(title: String) -> Post<Draft> {
        Post {
            title,
            state: PhantomData,
        }
    }

    fn publish(self) -> Post<Published> {
        Post {
            title: self.title,
            state: PhantomData,
        }
    }
}

impl Post<Published> {
    fn show(&self) {
        println!("Published post: {}", self.title);
    }
}

pub fn run() {
    println!("\n16. PhantomData marker state");

    let draft = Post::<Draft>::new(String::from("Generics in Rust"));
    let published = draft.publish();

    published.show();
}
