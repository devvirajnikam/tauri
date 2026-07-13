// Scenario:
// Library code and application code usually need different error styles.
//
// Thinking:
// Libraries should expose specific error types. Applications can often use
// broader error handling because they usually report/log instead of matching.

#[derive(Debug)]
enum LibraryError {
    InvalidInput,
    NotFound,
}

fn library_function(id: u32) -> Result<String, LibraryError> {
    match id {
        0 => Err(LibraryError::InvalidInput),
        1 => Ok(String::from("record")),
        _ => Err(LibraryError::NotFound),
    }
}

fn application_flow(id: u32) -> Result<(), String> {
    let record = library_function(id).map_err(|error| format!("operation failed: {:?}", error))?;
    println!("Loaded record: {}", record);
    Ok(())
}

pub fn run() {
    println!("\n11. Library errors vs app errors");

    println!("App flow success: {:?}", application_flow(1));
    println!("App flow failure: {:?}", application_flow(2));
}
