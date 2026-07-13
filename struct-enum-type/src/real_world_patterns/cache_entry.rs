// Scenario:
// A cache stores a value, but the app also needs to know whether the value is
// fresh, stale, or missing.
//
// Thinking:
// Use a struct for the cached value and metadata. Use an enum for freshness
// because the cache can be in exactly one known condition.

#[derive(Debug)]
enum CacheState {
    Fresh,
    Stale,
    Missing,
}

#[derive(Debug)]
struct CacheEntry<T> {
    value: Option<T>,
    state: CacheState,
}

impl<T> CacheEntry<T> {
    fn fresh(value: T) -> CacheEntry<T> {
        CacheEntry {
            value: Some(value),
            state: CacheState::Fresh,
        }
    }

    fn stale(value: T) -> CacheEntry<T> {
        CacheEntry {
            value: Some(value),
            state: CacheState::Stale,
        }
    }

    fn missing() -> CacheEntry<T> {
        CacheEntry {
            value: None,
            state: CacheState::Missing,
        }
    }
}

pub fn run() {
    println!("\n40. Cache entries with generic structs");

    let fresh_entry = CacheEntry::fresh(String::from("cached user"));
    let stale_entry = CacheEntry::stale(String::from("old settings"));
    let missing_entry: CacheEntry<String> = CacheEntry::missing();

    println!("{:?}", fresh_entry);
    println!(
        "Fresh fields -> value: {:?}, state: {:?}",
        fresh_entry.value, fresh_entry.state
    );
    println!("{:?}", stale_entry);
    println!("{:?}", missing_entry);
}
