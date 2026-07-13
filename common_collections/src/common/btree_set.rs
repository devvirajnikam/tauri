// Scenario:
// You need unique values and also want them in sorted order.
//
// Thinking:
// BTreeSet<T> is chosen when uniqueness and deterministic ordering are both
// important.

use std::collections::BTreeSet;

pub fn run() {
    println!("\n7. BTreeSet<T>");

    let mut invoice_numbers = BTreeSet::new();
    invoice_numbers.insert(300);
    invoice_numbers.insert(100);
    invoice_numbers.insert(200);
    invoice_numbers.insert(100);

    println!("Sorted unique invoice numbers: {:?}", invoice_numbers);
}
