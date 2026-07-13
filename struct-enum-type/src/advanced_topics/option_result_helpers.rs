fn parse_quantity(input: &str) -> Result<u32, String> {
    input
        .parse::<u32>()
        .map_err(|error| format!("invalid quantity: {}", error))
}

fn double_optional_number(number: Option<u32>) -> Option<u32> {
    number.map(|value| value * 2)
}

fn calculate_total(quantity_text: &str, price: u32) -> Result<u32, String> {
    let quantity = parse_quantity(quantity_text)?;
    Ok(quantity * price)
}

pub fn run() {
    println!("\n20. Option and Result helpers");

    let doubled = double_optional_number(Some(10));
    let fallback = double_optional_number(None).unwrap_or(0);

    println!("Option map result: {:?}", doubled);
    println!("Option unwrap_or fallback: {}", fallback);

    match calculate_total("4", 25) {
        Ok(total) => println!("Total is {}", total),
        Err(error) => println!("Error: {}", error),
    }

    match calculate_total("abc", 25) {
        Ok(total) => println!("Total is {}", total),
        Err(error) => println!("Error after map_err and ?: {}", error),
    }
}
