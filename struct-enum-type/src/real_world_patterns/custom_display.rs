// Scenario:
// Logs, CLI output, UI labels, and error messages need friendly text instead of
// raw debug output.
//
// Thinking:
// Debug is for developers. Display is for user-facing text. Implement Display
// when a struct or enum should control how it is printed with {}.

use std::fmt;

#[derive(Debug)]
enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentStatus::Pending => write!(formatter, "Pending"),
            PaymentStatus::Completed => write!(formatter, "Completed"),
            PaymentStatus::Failed => write!(formatter, "Failed"),
        }
    }
}

pub fn run() {
    println!("\n25. Custom Display");

    let statuses = [
        PaymentStatus::Pending,
        PaymentStatus::Completed,
        PaymentStatus::Failed,
    ];

    for status in statuses {
        println!("Debug: {:?}, Display: {}", status, status);
    }
}
