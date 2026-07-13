mod boxed_error;
mod error_source_chain;
mod library_vs_app_errors;
mod recovery_strategies;
mod thiserror_anyhow_overview;

pub fn run() {
    println!("\nAdvanced and real-world error design");

    boxed_error::run();
    error_source_chain::run();
    library_vs_app_errors::run();
    recovery_strategies::run();
    thiserror_anyhow_overview::run();
}
