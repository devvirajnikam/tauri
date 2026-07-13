// Scenario:
// A response can be success or failure, and both sides should choose their own
// data type.
//
// Thinking:
// Generic enums are useful when variants carry data but the data type changes by
// use case.

#[derive(Debug)]
enum ApiResult<T, E> {
    Success(T),
    Failure(E),
}

pub fn run() {
    println!("\n3. Generic enum");

    let success: ApiResult<String, String> = ApiResult::Success(String::from("saved"));
    let failure: ApiResult<String, String> = ApiResult::Failure(String::from("invalid input"));

    println!("Success response: {:?}", success);
    println!("Failure response: {:?}", failure);
}
