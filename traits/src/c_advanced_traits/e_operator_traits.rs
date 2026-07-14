// Scenario:
// A custom type should support operators like +.
//
// Thinking:
// Operator traits from std::ops define how operators behave for custom types.

use std::ops::Add;

#[derive(Debug)]
struct Money {
    cents: u32,
}

impl Add for Money {
    type Output = Money;

    fn add(self, right: Money) -> Self::Output {
        Money {
            cents: self.cents + right.cents,
        }
    }
}

pub fn run() {
    println!("\n14. Operator traits");

    let first = Money { cents: 500 };
    let second = Money { cents: 250 };
    let total = first + second;

    println!("Total money: {:?}", total);
}
