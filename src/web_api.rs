use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
}

pub async fn run() {
    println!("\n--- Web API (Axum) ---");
    println!("Starting server on http://127.0.0.1:3000");

    // 1. Define our routes
    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(get_status));

    // 2. Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server is running! Try running:");
    println!("  curl http://127.0.0.1:3000/status");

    // Await the server indefinitely
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Rust Learning API!"
}

async fn get_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: String::from("success"),
        message: String::from("The server is healthy and using Axum + Serde!"),
    })
}
