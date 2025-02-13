use axum::{body::Body, extract::{Request, State}};
use bytes::Bytes;
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use sqlx::{MySql, Pool, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie{
    pub title:String,
    pub year:i32,
    pub genre: String,
    pub rating:i32,
    pub id: Option<i32>,
}


pub async fn add_movie(State(pool): State<Pool<MySql>>, req: Request) -> String{
    let (parts, body) = req.into_parts();

    // Collect the body bytes properly
    let body_bytes: Bytes = match body.collect().await {
        Ok(full_body) => full_body.to_bytes(),
        Err(_) => Bytes::new(),
    };

    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Raw Body: {}", body_str);
    let mut movie:Movie;
    // Parse the body as JSON
    match from_slice::<Movie>(&body_bytes) {
        Ok(parsed) => {
            println!("Parsed Login Request: {:?}", parsed);
            movie = parsed;
        }
        Err(err) => {
            return format!("Failed to parse JSON: {}", err);
        }
    }

    if let Err(err) = sqlx::query("INSERT INTO movies (title, year, genre, rating) VALUES (?, ?, ?, ?)")
        .bind(&movie.title)
        .bind(&movie.year)
        .bind(&movie.genre)
        .bind(&movie.rating)
        .execute(&pool)
        .await
    {
        return format!("Database error: {}", err);
    }


    let query = "SELECT id, title, year, genre, rating FROM movies";
    match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => {
            let users: Vec<Movie> = rows.into_iter()
                .map(|row| Movie {
                    id: row.get(0),
                    title: row.get(1),
                    year: row.get(2),
                    genre: row.get(3),
                    rating: row.get(4),
                })
                .collect();

            match serde_json::to_string(&users) {
                Ok(json) => json,  // ✅ Return JSON string of users
                Err(_) => String::from("Failed to serialize users"),
            }
        }
        Err(err) => format!("Failed to fetch users: {}", err),
    }
    // Reconstruct the request with the consumed body
    // let new_body = Body::from(body_bytes);
    //String::from("Movie added")
}