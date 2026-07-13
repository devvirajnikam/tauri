// Scenario:
// API responses should have the same outer shape but different payload types.
//
// Thinking:
// Generic response wrappers keep the envelope consistent while allowing each
// endpoint to choose its own data type.

#[derive(Debug)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

pub fn run() {
    println!("\n7. Generic response wrapper");

    let user_response = ApiResponse {
        success: true,
        data: String::from("Asha"),
    };

    let count_response = ApiResponse {
        success: true,
        data: 25,
    };

    println!("User response: {:?}", user_response);
    println!(
        "User response fields -> success: {}, data: {}",
        user_response.success, user_response.data
    );
    println!("Count response: {:?}", count_response);
}
