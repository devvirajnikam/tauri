// Scenario:
// A custom collection wrapper should be buildable from iterators and extendable
// with more items.
//
// Thinking:
// FromIterator powers collect(). Extend powers extend(). Implementing these
// traits makes custom collection-like types feel natural in Rust.

#[derive(Debug)]
struct Names {
    values: Vec<String>,
}

impl FromIterator<String> for Names {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Names {
            values: iter.into_iter().collect(),
        }
    }
}

impl Extend<String> for Names {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        self.values.extend(iter);
    }
}

pub fn run() {
    println!("\n24. FromIterator and Extend");

    let mut names: Names = [String::from("Asha"), String::from("Ravi")]
        .into_iter()
        .collect();

    names.extend([String::from("Nina")]);

    println!("Names wrapper: {:?}", names);
}
