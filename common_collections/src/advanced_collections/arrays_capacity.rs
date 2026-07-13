// Scenario:
// A collection needs predictable size behavior or preallocated capacity.
//
// Thinking:
// Arrays are fixed-size. Vec::with_capacity reserves space ahead of time when
// you know roughly how many items will be pushed.

pub fn run() {
    println!("\n29. Fixed arrays and Vec capacity");

    let fixed_flags = [false; 4];
    println!("Fixed flags: {:?}", fixed_flags);

    let mut names = Vec::with_capacity(3);
    println!("Initial capacity: {}", names.capacity());

    names.push("Asha");
    names.push("Ravi");
    names.push("Nina");

    println!("Names: {:?}", names);
    println!("Final capacity: {}", names.capacity());
}
