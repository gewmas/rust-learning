mod async_rust;
mod borrowing;
mod enums;
mod error_handling;
mod generics;
mod lifetimes;
mod ownership;
mod persistence;
mod serialization;
mod traits;
mod web_api;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Load .env if it exists
    dotenvy::dotenv().ok();

    println!("Welcome to Rust Mastery!");

    // Run the theory exercises
    ownership::run();
    borrowing::run();
    lifetimes::run();
    enums::run();
    traits::run();
    generics::run();
    error_handling::run();
    serialization::run();

    // Async theory
    async_rust::run().await;

    // Initialize Database (Rusqlite)
    let conn = persistence::init_db();
    let shared_db = Arc::new(Mutex::new(conn));

    // Start Web Server with Persistence
    web_api::run(shared_db).await;
}
