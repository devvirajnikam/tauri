// Scenario:
// An IP address can be either IPv4 or IPv6, and each version has different data.
//
// Thinking:
// IpAddr is an enum in the standard library for exactly this reason. Each
// variant carries the data shape needed for that version.

#[derive(Debug)]
enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_ip(ip: MyIpAddr) {
    match ip {
        MyIpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        MyIpAddr::V6(address) => println!("IPv6: {}", address),
    }
}

pub fn run() {
    println!("\n54. Recreated IpAddr enum");

    print_ip(MyIpAddr::V4(127, 0, 0, 1));
    print_ip(MyIpAddr::V6(String::from("::1")));
}
