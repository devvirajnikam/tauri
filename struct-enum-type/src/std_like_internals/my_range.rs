// Scenario:
// Loops and slices often need a start and an end boundary.
//
// Thinking:
// Range-like types are structs because they group boundary data together. The
// exact struct shape changes based on whether the range is exclusive, inclusive,
// starts from zero, or has no upper bound.

#[derive(Debug)]
struct MyRange {
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct MyRangeInclusive {
    start: i32,
    end: i32,
}

impl MyRange {
    fn contains(&self, value: i32) -> bool {
        value >= self.start && value < self.end
    }
}

impl MyRangeInclusive {
    fn contains(&self, value: i32) -> bool {
        value >= self.start && value <= self.end
    }
}

pub fn run() {
    println!("\n49. Recreated Range structs");

    let exclusive = MyRange { start: 1, end: 5 };
    let inclusive = MyRangeInclusive { start: 1, end: 5 };

    println!("{:?} contains 5: {}", exclusive, exclusive.contains(5));
    println!("{:?} contains 5: {}", inclusive, inclusive.contains(5));
}
