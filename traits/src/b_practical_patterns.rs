mod a_validator_trait;
mod b_repository_trait;
mod c_service_trait;
mod d_strategy_trait;
mod e_formatter_trait;

pub fn run() {
    println!("\nPractical trait patterns");

    a_validator_trait::run();
    b_repository_trait::run();
    c_service_trait::run();
    d_strategy_trait::run();
    e_formatter_trait::run();
}
