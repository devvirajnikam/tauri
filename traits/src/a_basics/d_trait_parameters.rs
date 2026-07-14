// Scenario:
// A function should accept any value that implements a trait, without naming the
// generic type directly.
//
// Thinking:
// impl Trait in parameters is a shorter way to write simple trait bounds.

trait Notifier {
    fn notify(&self, message: &str);
}

struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("Email notification: {}", message);
    }
}

fn send_alert(notifier: &impl Notifier) {
    notifier.notify("stock is low");
}

pub fn run() {
    println!("\n4. impl Trait parameters");

    let email = EmailNotifier;
    send_alert(&email);
}
