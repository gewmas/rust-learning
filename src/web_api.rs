use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    username: String,
}

pub async fn run() {
    println!("\n--- Web API (Axum) ---");
    println!("Starting server on http://127.0.0.1:3000");

    // 1. Define our routes
    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(get_status))
        .route("/users", post(create_user));

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

async fn create_user(
    // Axum extractor: This automatically deserializes the JSON body!
    Json(payload): Json<CreateUser>,
) -> Json<UserResponse> {
    println!("Received request to create user: {}", payload.username);

    // In a real app, you'd save this to a database
    Json(UserResponse {
        id: 1337,
        username: payload.username,
    })
}
