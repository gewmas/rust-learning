mod async_rust;
mod borrowing;
mod enums;
mod error_handling;
mod generics;
mod lifetimes;
mod ownership;
mod serialization;
mod traits;
mod web_api;

#[tokio::main]
async fn main() {
    println!("Welcome to Rust Mastery!");

    // Run the completed exercises
    ownership::run();
    borrowing::run();
    lifetimes::run();
    enums::run();
    traits::run();
    generics::run();
    error_handling::run();
    serialization::run();

    // Async exercises
    async_rust::run().await;
    web_api::run().await;
}
