#[derive(Debug)]
struct Address {
    city: String,
    country: String,
}

#[derive(Debug)]
struct Customer {
    name: String,
    address: Address,
    status: CustomerStatus,
}

#[derive(Debug)]
enum CustomerStatus {
    Active,
    Blocked { reason: String },
}

pub fn run() {
    println!("\n15. Nested structs and enums");

    let active_customer = Customer {
        name: String::from("Ravi"),
        address: Address {
            city: String::from("Mumbai"),
            country: String::from("India"),
        },
        status: CustomerStatus::Active,
    };

    let blocked_customer = Customer {
        name: String::from("Nina"),
        address: Address {
            city: String::from("Pune"),
            country: String::from("India"),
        },
        status: CustomerStatus::Blocked {
            reason: String::from("Payment pending"),
        },
    };

    println!("{:?}", active_customer);
    println!(
        "Active customer fields -> name: {}, city: {}, country: {}, status: {:?}",
        active_customer.name,
        active_customer.address.city,
        active_customer.address.country,
        active_customer.status
    );
    match blocked_customer.status {
        CustomerStatus::Active => println!("{} is active", blocked_customer.name),
        CustomerStatus::Blocked { reason } => {
            println!("{} is blocked: {}", blocked_customer.name, reason);
        }
    }
}
