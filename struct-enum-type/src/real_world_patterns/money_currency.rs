// Scenario:
// Money should not be passed around as a plain number, because the number needs
// currency information and predictable formatting.
//
// Thinking:
// Use a struct to keep amount and currency together. Use an enum for supported
// currencies so invalid currency strings do not spread through the code.

use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Currency {
    Inr,
    Usd,
}

#[derive(Debug, Clone, Copy)]
struct Money {
    amount_in_minor_units: u32,
    currency: Currency,
}

impl Money {
    fn new(amount_in_minor_units: u32, currency: Currency) -> Money {
        Money {
            amount_in_minor_units,
            currency,
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = self.amount_in_minor_units / 100;
        let minor = self.amount_in_minor_units % 100;
        let symbol = match self.currency {
            Currency::Inr => "INR",
            Currency::Usd => "USD",
        };

        write!(formatter, "{} {}.{:02}", symbol, major, minor)
    }
}

pub fn run() {
    println!("\n36. Money structs and currency enums");

    let invoice_total = Money::new(123_450, Currency::Inr);
    let subscription_price = Money::new(2_999, Currency::Usd);

    println!("Invoice total: {}", invoice_total);
    println!("Subscription price: {}", subscription_price);
}
