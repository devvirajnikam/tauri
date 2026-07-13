// Scenario:
// A UI, CLI, or API can ask the app to perform several different actions:
// create, rename, delete, or print something.
//
// Thinking:
// Use an enum for commands because only one action is requested at a time. Each
// variant carries exactly the fields needed for that action.

#[derive(Debug)]
enum UserCommand {
    Create { name: String },
    Rename { id: u32, new_name: String },
    Delete { id: u32 },
    PrintAll,
}

fn handle_command(command: UserCommand) {
    match command {
        UserCommand::Create { name } => println!("Create user named {}", name),
        UserCommand::Rename { id, new_name } => println!("Rename user {} to {}", id, new_name),
        UserCommand::Delete { id } => println!("Delete user {}", id),
        UserCommand::PrintAll => println!("Print all users"),
    }
}

pub fn run() {
    println!("\n33. Command enums");

    let commands = [
        UserCommand::Create {
            name: String::from("John"),
        },
        UserCommand::Rename {
            id: 1,
            new_name: String::from("Jane"),
        },
        UserCommand::Delete { id: 2 },
        UserCommand::PrintAll,
    ];

    for command in commands {
        handle_command(command);
    }
}
