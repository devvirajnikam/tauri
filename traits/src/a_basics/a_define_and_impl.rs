// Scenario:
// Different types need to expose the same behavior.
//
// Thinking:
// A trait defines behavior. Each type implements that behavior in its own way.

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Video {
    title: String,
    duration_minutes: u32,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article '{}' by {}", self.title, self.author)
    }
}

impl Summary for Video {
    fn summarize(&self) -> String {
        format!("Video '{}' - {} minutes", self.title, self.duration_minutes)
    }
}

pub fn run() {
    println!("\n1. Define and implement a trait");

    let article = Article {
        title: String::from("Traits in Rust"),
        author: String::from("Asha"),
    };
    let video = Video {
        title: String::from("Rust trait basics"),
        duration_minutes: 12,
    };

    println!("{}", article.summarize());
    println!("{}", video.summarize());
}
