// Scenario:
// A function can usually borrow text, but sometimes it needs to create an owned
// cleaned version.
//
// Thinking:
// Cow<'a, str> means "clone on write". It avoids allocation when borrowed data
// can be reused, but allows owned data when changes are needed.

use std::borrow::Cow;

fn normalize_name(name: &str) -> Cow<'_, str> {
    let trimmed = name.trim();

    if trimmed == name {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(trimmed.to_string())
    }
}

pub fn run() {
    println!("\n19. Cow<'a, str>");

    let clean = normalize_name("Asha");
    let cleaned = normalize_name("  Ravi  ");

    println!("Clean name: {}", clean);
    println!("Cleaned name: {}", cleaned);
}
