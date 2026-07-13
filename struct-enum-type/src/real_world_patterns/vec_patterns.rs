// Scenario:
// An invoice has one status and many line items.
//
// Thinking:
// Use a struct for the invoice because it groups related data. Use Vec<LineItem>
// because the number of items can grow. Use an enum because status must be one
// known value, such as Draft or Paid.

#[derive(Debug)]
struct LineItem {
    name: String,
    quantity: u32,
    price: u32,
}

#[derive(Debug)]
enum InvoiceStatus {
    Draft,
    Paid,
}

#[derive(Debug)]
struct Invoice {
    status: InvoiceStatus,
    items: Vec<LineItem>,
}

impl Invoice {
    fn total(&self) -> u32 {
        self.items
            .iter()
            .map(|item| item.quantity * item.price)
            .sum()
    }
}

pub fn run() {
    println!("\n22. Vec<Struct> and enum status fields");

    let invoice = Invoice {
        status: InvoiceStatus::Draft,
        items: vec![
            LineItem {
                name: String::from("Keyboard"),
                quantity: 2,
                price: 1500,
            },
            LineItem {
                name: String::from("Mouse"),
                quantity: 1,
                price: 700,
            },
        ],
    };

    println!("{:?}", invoice);
    println!("Invoice status: {:?}", invoice.status);
    for item in &invoice.items {
        println!(
            "Line item: {} x {} at {}",
            item.name, item.quantity, item.price
        );
    }
    println!("Invoice total: {}", invoice.total());

    let paid_status = InvoiceStatus::Paid;
    println!("Another possible status: {:?}", paid_status);
}
