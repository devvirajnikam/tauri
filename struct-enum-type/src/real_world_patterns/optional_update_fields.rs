// Scenario:
// A profile update form may update only the name, only the email, or both.
//
// Thinking:
// Use a separate update struct with Option fields. None means "do not change
// this field", while Some(value) means "replace the old value".

#[derive(Debug)]
struct UserProfile {
    name: String,
    email: String,
}

#[derive(Debug)]
struct UpdateUserProfile {
    name: Option<String>,
    email: Option<String>,
}

impl UserProfile {
    fn apply_update(&mut self, update: UpdateUserProfile) {
        if let Some(name) = update.name {
            self.name = name;
        }

        if let Some(email) = update.email {
            self.email = email;
        }
    }
}

pub fn run() {
    println!("\n38. Optional update structs");

    let mut profile = UserProfile {
        name: String::from("John"),
        email: String::from("john@example.com"),
    };

    profile.apply_update(UpdateUserProfile {
        name: Some(String::from("John Doe")),
        email: None,
    });

    println!("{:?}", profile);
}
