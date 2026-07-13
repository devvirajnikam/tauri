// Scenario:
// Async work may be ready now, or it may still be waiting for something.
//
// Thinking:
// Poll is an enum because a task has two states when checked: Ready(value) or
// Pending. This is one of the core ideas behind Rust async.

#[derive(Debug)]
enum MyPoll<T> {
    Ready(T),
    Pending,
}

fn check_job(progress: u8) -> MyPoll<String> {
    if progress >= 100 {
        MyPoll::Ready(String::from("job completed"))
    } else {
        MyPoll::Pending
    }
}

pub fn run() {
    println!("\n48. Recreated Poll enum");

    println!("Progress 40: {:?}", check_job(40));
    println!("Progress 100: {:?}", check_job(100));
}
