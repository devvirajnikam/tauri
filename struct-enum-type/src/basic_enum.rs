#[derive(Debug)]
enum Role {
    Admin,
    User,
    Guest,
}

#[derive(Debug)]
struct Person {
    name: String,
    role: Role,
}

pub fn run() {
    println!("\n3. Basic enum");

    let admin = Person {
        name: String::from("Asha"),
        role: Role::Admin,
    };

    let user = Person {
        name: String::from("Ravi"),
        role: Role::User,
    };

    let guest = Person {
        name: String::from("Meera"),
        role: Role::Guest,
    };

    println!("{:?}", admin);
    println!("{:?}", user);
    println!("{:?}", guest);
    println!("{} has role {:?}", admin.name, admin.role);
}
