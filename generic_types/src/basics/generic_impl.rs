// Scenario:
// A generic type needs methods that work for every possible T.
//
// Thinking:
// impl<T> attaches methods to all versions of the generic type.

#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Pair<T> {
        Pair { left, right }
    }

    fn into_tuple(self) -> (T, T) {
        (self.left, self.right)
    }
}

pub fn run() {
    println!("\n4. Generic impl block");

    let pair = Pair::new("left", "right");
    println!("Pair as tuple: {:?}", pair.into_tuple());
}
