// Scenario:
// A notification system may send the same message through email, SMS, push, or
// another channel chosen at runtime.
//
// Thinking:
// Use a trait for shared behavior. Use Box<dyn Trait> when a collection must
// hold different concrete types that all implement the same trait.

trait Notifier {
    fn send(&self, message: &str);
}

struct EmailNotifier {
    address: String,
}

struct SmsNotifier {
    phone_number: String,
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) {
        println!("Email to {}: {}", self.address, message);
    }
}

impl Notifier for SmsNotifier {
    fn send(&self, message: &str) {
        println!("SMS to {}: {}", self.phone_number, message);
    }
}

pub fn run() {
    println!("\n28. Trait objects with Box<dyn Trait>");

    let notifiers: Vec<Box<dyn Notifier>> = vec![
        Box::new(EmailNotifier {
            address: String::from("user@example.com"),
        }),
        Box::new(SmsNotifier {
            phone_number: String::from("+91-9999999999"),
        }),
    ];

    for notifier in notifiers {
        notifier.send("Your order has shipped");
    }
}
