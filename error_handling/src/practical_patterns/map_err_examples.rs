// Scenario:
// A low-level error should be converted into a message that makes sense for this
// part of the app.
//
// Thinking:
// map_err transforms only the error side of Result. The success value is left
// unchanged.

fn parse_port(input: &str) -> Result<u16, String> {
    input
        .parse::<u16>()
        .map_err(|error| format!("port must be a number between 0 and 65535: {}", error))
}

pub fn run() {
    println!("\n6. map_err");

    println!("Valid port: {:?}", parse_port("4317"));
    println!("Invalid port: {:?}", parse_port("not-a-port"));
}
