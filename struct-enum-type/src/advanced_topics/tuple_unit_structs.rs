#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct UserId(u32);

#[derive(Debug)]
struct AlwaysAllowed;

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
}

pub fn run() {
    println!("\n9. Tuple structs, unit structs, and struct update syntax");

    let red = Color(255, 0, 0);
    let user_id = UserId(7);
    let permission = AlwaysAllowed;

    println!("Tuple struct Color: {:?}", red);
    println!(
        "Color channels: red={}, green={}, blue={}",
        red.0, red.1, red.2
    );
    println!("Tuple struct UserId: {:?}", user_id);
    println!("User id value: {}", user_id.0);
    println!("Unit struct: {:?}", permission);

    let first_user = User {
        id: UserId(1),
        name: String::from("John"),
    };

    let renamed_user = User {
        name: String::from("Jane"),
        ..first_user
    };

    println!("Updated user: {:?}", renamed_user);
    println!(
        "Updated user fields -> id: {:?}, name: {}",
        renamed_user.id, renamed_user.name
    );
}
