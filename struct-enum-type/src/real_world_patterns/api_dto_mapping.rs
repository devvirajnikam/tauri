// Scenario:
// Database/domain models often contain private internal fields, while API
// responses should expose only safe public fields.
//
// Thinking:
// Keep separate structs for domain data and response DTOs. Then use From to make
// the mapping explicit, repeatable, and hard to accidentally bypass.

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    password_hash: String,
}

#[derive(Debug)]
struct UserDto {
    id: u32,
    display_name: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            display_name: user.name,
        }
    }
}

pub fn run() {
    println!("\n32. API DTO mapping");

    let user = User {
        id: 1,
        name: String::from("Asha"),
        password_hash: String::from("hashed-secret"),
    };

    println!(
        "Internal password hash length: {}",
        user.password_hash.len()
    );

    let dto = UserDto::from(user);

    println!("{:?}", dto);
    println!(
        "DTO fields -> id: {}, display_name: {}",
        dto.id, dto.display_name
    );
}
