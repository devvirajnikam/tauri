// Scenario:
// A workflow has multiple steps, and any step may fail.
//
// Thinking:
// The ? operator returns early on Err and unwraps the Ok value. It keeps
// recoverable error code short without hiding the error path.

fn parse_quantity(input: &str) -> Result<u32, String> {
    input
        .parse::<u32>()
        .map_err(|error| format!("invalid quantity: {}", error))
}

fn calculate_total(quantity_text: &str, price: u32) -> Result<u32, String> {
    let quantity = parse_quantity(quantity_text)?;
    Ok(quantity * price)
}

pub fn run() {
    println!("\n5. The ? operator");

    println!("Valid total: {:?}", calculate_total("4", 25));
    println!("Invalid total: {:?}", calculate_total("abc", 25));
}
