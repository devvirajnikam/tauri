#[derive(Debug)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn status_message(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::Pending => "Order is waiting for payment",
        OrderStatus::Paid => "Order payment is complete",
        OrderStatus::Shipped => "Order is on the way",
        OrderStatus::Cancelled => "Order was cancelled",
    }
}

pub fn run() {
    println!("\n5. match");

    let statuses = [
        OrderStatus::Pending,
        OrderStatus::Paid,
        OrderStatus::Shipped,
        OrderStatus::Cancelled,
    ];

    for status in &statuses {
        println!("{:?}: {}", status, status_message(&status));
    }
}
