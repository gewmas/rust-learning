use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    id: i64,
    username: String,
}

type DbState = Arc<Mutex<rusqlite::Connection>>;

/**
 *  1. Create a user:

curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"username": "rust_ace", "email": "ace@example.com"}'

   2. Verify they are saved:

curl http://127.0.0.1:3000/users

 */
pub async fn run(db: DbState) {
    println!("\n--- Web API (Axum + Rusqlite) ---");
    println!("Starting server on http://127.0.0.1:3000");

    let app = Router::new()
        .route("/", get(root))
        .route("/status", get(get_status))
        .route("/users", post(create_user).get(get_users))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server is running! Try running:");
    println!("  curl -X POST http://127.0.0.1:3000/users -H \"Content-Type: application/json\" -d '{{\"username\": \"rust_user\", \"email\": \"rust@example.com\"}}'");
    println!("  curl http://127.0.0.1:3000/users");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Rust Persistence API!"
}

async fn get_status() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: String::from("success"),
        message: String::from("The server is healthy and using Axum + Rusqlite!"),
    })
}

async fn create_user(
    State(db): State<DbState>,
    Json(payload): Json<CreateUser>,
) -> Json<UserResponse> {
    println!("Creating user in database: {}", payload.username);

    // We use spawn_blocking because rusqlite is a synchronous library.
    // This is a crucial pattern for senior Rust developers.
    let res = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (username, email) VALUES (?1, ?2)",
            params![payload.username, payload.email],
        )
        .map(|_| {
            let id = conn.last_insert_rowid();
            UserResponse {
                id,
                username: payload.username,
            }
        })
    })
    .await
    .unwrap()
    .unwrap();

    Json(res)
}

async fn get_users(State(db): State<DbState>) -> Json<Vec<UserResponse>> {
    let users = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, username FROM users").unwrap();
        let user_iter = stmt
            .query_map([], |row| {
                Ok(UserResponse {
                    id: row.get(0)?,
                    username: row.get(1)?,
                })
            })
            .unwrap();

        user_iter.map(|u| u.unwrap()).collect::<Vec<_>>()
    })
    .await
    .unwrap();

    Json(users)
}
