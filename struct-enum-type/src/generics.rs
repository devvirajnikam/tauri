#[derive(Debug)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

#[derive(Debug)]
enum ApiResult<T, E> {
    Success(T),
    Failure(E),
}

pub fn run() {
    println!("\n7. Generics with structs and enums");

    let user_response = ApiResponse {
        success: true,
        data: String::from("John"),
    };

    let count_response = ApiResponse {
        success: true,
        data: 25,
    };

    let success_result: ApiResult<String, String> = ApiResult::Success(String::from("Saved"));
    let failure_result: ApiResult<String, String> =
        ApiResult::Failure(String::from("Invalid data"));

    println!("{:?}", user_response);
    println!(
        "User response fields -> success: {}, data: {}",
        user_response.success, user_response.data
    );
    println!("{:?}", count_response);
    println!("{:?}", success_result);
    println!("{:?}", failure_result);
}
