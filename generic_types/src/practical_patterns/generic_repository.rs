// Scenario:
// A repository should work with different entity types.
//
// Thinking:
// A generic trait can describe storage behavior once and let each repository
// choose the entity type.

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

trait Repository<T> {
    fn find_all(&self) -> Vec<T>;
}

struct UserRepository {
    users: Vec<User>,
}

impl Repository<User> for UserRepository {
    fn find_all(&self) -> Vec<User> {
        self.users.clone()
    }
}

pub fn run() {
    println!("\n9. Generic repository trait");

    let repository = UserRepository {
        users: vec![User {
            id: 1,
            name: String::from("Asha"),
        }],
    };

    let users = repository.find_all();
    println!("Users: {:?}", users);
    for user in users {
        println!("User fields -> id: {}, name: {}", user.id, user.name);
    }
}
