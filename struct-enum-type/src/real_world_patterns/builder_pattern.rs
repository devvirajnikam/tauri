// Scenario:
// A config object has several fields, defaults, and optional overrides.
//
// Thinking:
// Use a builder when direct struct creation becomes noisy or unclear. Builder
// methods let callers choose only the fields they want to change before build().

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    use_tls: bool,
}

#[derive(Debug)]
struct ServerConfigBuilder {
    host: String,
    port: u16,
    use_tls: bool,
}

impl ServerConfigBuilder {
    fn new() -> ServerConfigBuilder {
        ServerConfigBuilder {
            host: String::from("127.0.0.1"),
            port: 8080,
            use_tls: false,
        }
    }

    fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn use_tls(mut self, use_tls: bool) -> Self {
        self.use_tls = use_tls;
        self
    }

    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            use_tls: self.use_tls,
        }
    }
}

pub fn run() {
    println!("\n30. Builder pattern for structs");

    let config = ServerConfigBuilder::new()
        .host(String::from("localhost"))
        .port(3000)
        .use_tls(true)
        .build();

    println!("{:?}", config);
    println!(
        "Server config fields -> host: {}, port: {}, use_tls: {}",
        config.host, config.port, config.use_tls
    );
}
