use axum::{
    extract::{Request, State},
};
use bytes::Bytes;
use http_body_util::BodyExt;
use serde::{Serialize, Deserialize};
use serde_json::from_slice;
use sqlx::{ MySql, Pool, Row};

#[derive(Debug, Deserialize, Serialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub csrf_token: Option<String>  // Consider hashing passwords instead of storing plain text.
}

pub async fn register(State(pool): State<Pool<MySql>>, req: Request) -> String {
    let (_, body) = req.into_parts();

    // Collect the body bytes
    let body_bytes: Bytes = match body.collect().await {
        Ok(full_body) => full_body.to_bytes(),
        Err(_) => return String::from("Failed to collect body"),
    };

    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Raw Body: {}", body_str);

    // Parse JSON into RegisterRequest struct
    let curr_user: RegisterRequest = match from_slice::<RegisterRequest>(&body_bytes) {
        Ok(parsed) => parsed,
        Err(err) => return format!("Failed to parse JSON: {}", err),
    };

    println!("Current User: {} {}", curr_user.username, curr_user.password);

    // Insert into DB and return early if there's an error
    if let Err(err) = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&curr_user.username)
        .bind(&curr_user.password)
        .execute(&pool)
        .await
    {
        return format!("Database error: {}", err);
    }

    // Fetch users after insertion
    let query = "SELECT id, username, password, csrf_token FROM users";
    match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => {
            let users: Vec<User> = rows.into_iter()
                .map(|row| User {
                    id: row.get(0),
                    username: row.get(1),
                    password: row.get(2),
                    csrf_token: row.get(3)
                })
                .collect();

            match serde_json::to_string(&users) {
                Ok(json) => json,  // ✅ Return JSON string of users
                Err(_) => String::from("Failed to serialize users"),
            }
        }
        Err(err) => format!("Failed to fetch users: {}", err),
    }
}
