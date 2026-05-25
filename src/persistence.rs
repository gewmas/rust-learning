use rusqlite::Connection;

pub fn init_db() -> Connection {
    let conn = Connection::open("sqlite.db").expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        (),
    )
    .expect("Failed to create users table");

    conn
}
