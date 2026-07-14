// Scenario:
// Business code should not care where data is stored.
//
// Thinking:
// A repository trait hides storage details behind behavior.

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
    println!("\n6. Repository trait");

    let repository = InMemoryUserRepository {
        users: vec![User {
            id: 1,
            name: String::from("Nina"),
        }],
    };

    print_user(&repository, 1);
    print_user(&repository, 99);
}
