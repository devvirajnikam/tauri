// Scenario:
// Calling an outside service can succeed, fail permanently, or fail temporarily
// and need retry.
//
// Thinking:
// Use an enum when different outcomes need different data and handling. This
// makes retry logic explicit instead of hiding it behind strings or booleans.

#[derive(Debug)]
enum ServiceCallResult<T> {
    Success(T),
    RetryLater { after_seconds: u32 },
    PermanentFailure { message: String },
}

fn call_payment_service(amount: u32) -> ServiceCallResult<String> {
    if amount == 0 {
        ServiceCallResult::PermanentFailure {
            message: String::from("amount must be greater than zero"),
        }
    } else if amount > 10_000 {
        ServiceCallResult::RetryLater { after_seconds: 30 }
    } else {
        ServiceCallResult::Success(String::from("PAYMENT-OK"))
    }
}

pub fn run() {
    println!("\n43. External service result enums");

    let results = [
        call_payment_service(500),
        call_payment_service(15_000),
        call_payment_service(0),
    ];

    for result in results {
        match result {
            ServiceCallResult::Success(reference) => {
                println!("Payment service success: {}", reference);
            }
            ServiceCallResult::RetryLater { after_seconds } => {
                println!("Retry service call after {} seconds", after_seconds);
            }
            ServiceCallResult::PermanentFailure { message } => {
                println!("Do not retry: {}", message);
            }
        }
    }
}
