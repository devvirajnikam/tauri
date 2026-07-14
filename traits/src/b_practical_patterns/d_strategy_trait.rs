// Scenario:
// Different algorithms should be interchangeable.
//
// Thinking:
// The strategy pattern uses a trait to make algorithms swappable.

trait DiscountStrategy {
    fn discount(&self, amount: u32) -> u32;
}

struct NoDiscount;

impl DiscountStrategy for NoDiscount {
    fn discount(&self, amount: u32) -> u32 {
        amount
    }
}

struct TenPercentDiscount;

impl DiscountStrategy for TenPercentDiscount {
    fn discount(&self, amount: u32) -> u32 {
        amount - (amount / 10)
    }
}

fn final_amount(strategy: &impl DiscountStrategy, amount: u32) -> u32 {
    strategy.discount(amount)
}

pub fn run() {
    println!("\n8. Strategy trait");

    println!("No discount: {}", final_amount(&NoDiscount, 1000));
    println!(
        "10 percent discount: {}",
        final_amount(&TenPercentDiscount, 1000)
    );
}
