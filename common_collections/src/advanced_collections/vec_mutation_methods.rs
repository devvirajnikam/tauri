// Scenario:
// A Vec needs to remove, keep, split, or drain items after it has been built.
//
// Thinking:
// Vec has mutation methods that avoid manual index loops. retain keeps matching
// items, drain removes a range, and split_off moves the tail into a new Vec.

pub fn run() {
    println!("\n26. Vec mutation methods");

    let mut numbers = vec![1, 2, 3, 4, 5, 6];
    numbers.retain(|number| number % 2 == 0);
    println!("After retain even: {:?}", numbers);

    let drained: Vec<i32> = numbers.drain(0..1).collect();
    println!("Drained values: {:?}", drained);
    println!("After drain: {:?}", numbers);

    let mut letters = vec!["a", "b", "c", "d"];
    let tail = letters.split_off(2);
    println!("Letters head: {:?}", letters);
    println!("Letters tail: {:?}", tail);
}
