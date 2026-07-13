#[derive(Debug)]
struct DraftPost {
    title: String,
}

#[derive(Debug)]
struct PublishedPost {
    title: String,
}

impl DraftPost {
    fn new(title: String) -> DraftPost {
        DraftPost { title }
    }

    fn publish(self) -> PublishedPost {
        PublishedPost { title: self.title }
    }
}

pub fn run() {
    println!("\n12. Methods that consume self");

    let draft = DraftPost::new(String::from("Rust structs and enums"));
    let published = draft.publish();

    println!("Published post: {:?}", published);
    println!("Published title: {}", published.title);
}
