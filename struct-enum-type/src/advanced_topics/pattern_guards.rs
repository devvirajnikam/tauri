#[derive(Debug)]
enum LoginAttempt {
    Success { user_id: u32 },
    Failed { reason: String, count: u8 },
}

pub fn run() {
    println!("\n19. Pattern matching guards");

    let attempts = [
        LoginAttempt::Success { user_id: 10 },
        LoginAttempt::Failed {
            reason: String::from("wrong password"),
            count: 1,
        },
        LoginAttempt::Failed {
            reason: String::from("wrong password"),
            count: 5,
        },
    ];

    for attempt in attempts {
        match attempt {
            LoginAttempt::Success { user_id } => println!("User {} logged in", user_id),
            LoginAttempt::Failed { reason, count } if count >= 3 => {
                println!("Account warning after {} failures: {}", count, reason);
            }
            LoginAttempt::Failed { reason, count } => {
                println!("Login failed {} time(s): {}", count, reason);
            }
        }
    }
}
