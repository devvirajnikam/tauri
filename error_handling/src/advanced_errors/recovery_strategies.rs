// Scenario:
// Not every error should stop the whole operation.
//
// Thinking:
// Some errors can use fallback values, some should be retried, and some should
// stop immediately. Encoding that decision keeps recovery logic explicit.

#[derive(Debug)]
enum FetchError {
    Temporary,
    Permanent,
}

fn fetch_from_cache(key: &str) -> Result<String, FetchError> {
    if key == "user:1" {
        Ok(String::from("cached user"))
    } else if key == "retry" {
        Err(FetchError::Temporary)
    } else {
        Err(FetchError::Permanent)
    }
}

pub fn run() {
    println!("\n12. Recovery strategies");

    for key in ["user:1", "retry", "missing"] {
        match fetch_from_cache(key) {
            Ok(value) => println!("{} -> {}", key, value),
            Err(FetchError::Temporary) => println!("{} -> retry later", key),
            Err(FetchError::Permanent) => println!("{} -> use fallback value", key),
        }
    }
}
