use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
    is_active: bool,
}

pub fn run() {
    println!("\n--- Serialization (Serde) ---");

    // 1. Create an instance of our struct
    let user = User {
        id: 1,
        username: String::from("rust_ace"),
        email: String::from("ace@example.com"),
        is_active: true,
    };

    // 2. Serialize: Struct -> JSON String
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("Serialized JSON:\n{}", json);

    // 3. Deserialize: JSON String -> Struct
    let json_data = r#"
        {
            "id": 2,
            "username": "coder_x",
            "email": "x@example.com",
            "is_active": false
        }
    "#;

    let deserialized_user: User = serde_json::from_str(json_data).unwrap();
    println!("Deserialized Struct: {:?}", deserialized_user);
    println!(
        "Username of deserialized user: {}",
        deserialized_user.username
    );
}
