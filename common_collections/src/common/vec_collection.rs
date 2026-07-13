// Scenario:
// You need a growable list of values, such as cart items, numbers, or names.
//
// Thinking:
// Vec<T> is the default collection when order matters and you mostly add/remove
// items at the end.

pub fn run() {
    println!("\n1. Vec<T>");

    let mut scores = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);

    println!("Scores: {:?}", scores);
    println!("Length: {}", scores.len());
    println!("First score: {:?}", scores.get(0));

    for score in &scores {
        println!("Score by reference: {}", score);
    }

    let doubled_scores: Vec<i32> = scores.iter().map(|score| score * 2).collect();
    println!("Doubled scores: {:?}", doubled_scores);
}
