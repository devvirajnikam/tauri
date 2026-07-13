// Scenario:
// Internal app data often has a different shape than the data returned by an API.
//
// Thinking:
// Implement From when one type can be converted into another without failing.
// Rust automatically gives Into from that From implementation.

#[derive(Debug)]
struct User {
    name: String,
}

#[derive(Debug)]
struct UserResponse {
    display_name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            display_name: user.name,
        }
    }
}

pub fn run() {
    println!("\n23. From and Into conversions");

    let user = User {
        name: String::from("John"),
    };

    let response: UserResponse = user.into();

    println!("{:?}", response);
    println!("Display name: {}", response.display_name);
}
