// Scenario:
// Business logic should not care whether users come from memory, a database, or
// an API.
//
// Thinking:
// Use a trait for the repository behavior. Then structs can implement that trait
// for different storage backends without changing the calling code.

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

trait UserRepository {
    fn find_by_id(&self, id: u32) -> Option<User>;
}

struct InMemoryUserRepository {
    users: Vec<User>,
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: u32) -> Option<User> {
        self.users.iter().find(|user| user.id == id).cloned()
    }
}

fn print_user(repository: &impl UserRepository, id: u32) {
    match repository.find_by_id(id) {
        Some(user) => println!("Found user: {:?}, name={}", user, user.name),
        None => println!("User {} not found", id),
    }
}

pub fn run() {
    println!("\n37. Repository traits");

    let repository = InMemoryUserRepository {
        users: vec![User {
            id: 1,
            name: String::from("John"),
        }],
    };

    print_user(&repository, 1);
    print_user(&repository, 99);
}
