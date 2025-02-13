use axum::{routing::get, Router, extract::State, response::Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

pub mod handlers;
pub mod routes;
pub mod middlewares;

use sqlx::mysql::{MySqlPool};
use sqlx::{MySql, Row, Pool, Executor};
use tokio;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub async fn database_connection()-> Result<MySqlPool, sqlx::Error> {
    // Try to establish a database connection
    let pool = match MySqlPool::connect("mysql://root@127.0.0.1:4000/movies").await {
        Ok(pool) => {
            println!("Database connection successful...");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to database: {}", err);
            return Err(err);
        }
    };

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            password VARCHAR(255) NOT NULL,
            csrf_token VARCHAR(255)
        )
        "#
    ).await?;

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS movies (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            year INT(4),
            genre VARCHAR(255),
            rating INT
        )
        "#
    ).await?;

    Ok(pool)

}


// async fn main() -> Result<(), sqlx::Error> {
//     // Connect to TiDB (which uses TiKV for storage)
//    // let pool = MySqlPool::connect("mysql://root@127.0.0.1:4000").await?;

//     // Create a table
//     sqlx::query("CREATE TABLE IF NOT EXISTS users (id INT PRIMARY KEY, name VARCHAR(255))")
//         .execute(&pool)
//         .await?;

//     // Insert a row
//     sqlx::query("INSERT INTO users (id, name) VALUES (?, ?)")
//         .bind(1)
//         .bind("Alice")
//         .execute(&pool)
//         .await?;

//     // Fetch data
//     let row = sqlx::query("SELECT name FROM users WHERE id = ?")
//         .bind(1)
//         .fetch_one(&pool)
//         .await?;

//     let name: String = row.get("name");
//     println!("User: {}", name); // Output: Alice

//     Ok(())
// }








// #[tokio::main]
// async fn main() -> Result<()> {

//     let app = Router::new()
//         .route("/get/:key", get(get_value))
//         .route("/set/:key/:value", get(set_value))
//         .with_state(app_state);

//     // Start Axum server
//     axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();

//     Ok(())
// }

// Shared App State


// Route to Get a Value from TiKV
// async fn get_value(
//     State(state): State<Arc<AppState>>,
//     axum::extract::Path(key): axum::extract::Path<String>,
// ) -> Json<serde_json::Value> {
//     let client = state.client.lock().await;
//     match client.get(key.clone()).await {
//         Ok(Some(value)) => Json(json!({"key": key, "value": String::from_utf8_lossy(&value)})),
//         Ok(None) => Json(json!({"error": "Key not found"})),
//         Err(e) => Json(json!({"error": e.to_string()})),
//     }
// }

// // Route to Set a Value in TiKV
// async fn set_value(
//     State(state): State<Arc<AppState>>,
//     axum::extract::Path((key, value)): axum::extract::Path<(String, String)>,
// ) -> Json<serde_json::Value> {
//     let client = state.client.lock().await;
//     match client.put(key.clone(), value.clone()).await {
//         Ok(_) => Json(json!({"message": "Value set successfully"})),
//         Err(e) => Json(json!({"error": e.to_string()})),
//     }
// }
