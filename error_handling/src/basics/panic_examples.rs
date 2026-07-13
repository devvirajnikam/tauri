// Scenario:
// Some failures mean the program cannot safely continue.
//
// Thinking:
// panic! is for unrecoverable bugs or impossible states. For expected failures,
// prefer Result so the caller can decide what to do.

fn get_required_config(key: &str) -> &str {
    match key {
        "APP_NAME" => "error-handling-demo",
        _ => panic!("missing required config: {}", key),
    }
}

pub fn run() {
    println!("\n3. panic! for unrecoverable errors");

    let app_name = get_required_config("APP_NAME");
    println!("Required config value: {}", app_name);
    println!("The panic path is not called here so cargo run can continue.");
}
