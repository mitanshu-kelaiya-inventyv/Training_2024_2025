use axum::{extract::{Path, Request, State}, http::HeaderMap, routing::get, Json, Router};
use serde::{Serialize, Deserialize};
use sqlx::{MySql, Pool, Row};
use std::net::SocketAddr;

use crate::handlers::add_movie::Movie;

pub async fn get_all(State(pool): State<Pool<MySql>>,
req: Request) -> String
{
    let headers: &HeaderMap = req.headers();
        
    for (key, value) in headers.iter() {
        println!("Header: {} = {:?}", key, value);
    }
    
let query = "SELECT * FROM movies";
match sqlx::query(query).fetch_all(&pool).await {
    Ok(rows) => {
        let movies: Vec<Movie> = rows.into_iter()
            .map(|row| Movie {
                id: row.get(0),
                title: row.get(1),
                year: row.get(2),
                genre: row.get(3),
                rating: row.get(4)
            })
            .collect();

        match serde_json::to_string(&movies) {
            Ok(json) => json,  // ✅ Return JSON string of users
            Err(_) => String::from("Failed to serialize users"),
        }
    }
    Err(err) => format!("Failed to fetch users: {}", err),
}

    // Json(movies)

}

pub async fn get_by_year(
    State(pool): State<Pool<MySql>>, 
    Path(year): Path<i32>,    
    _req: Request
) -> String {
    match sqlx::query("SELECT id, title, year, genre, rating FROM movies WHERE year = ?")
        .bind(year)
        .fetch_all(&pool)  
        .await
    {
        Ok(rows) => {
            let movies: Vec<Movie> = rows.into_iter()
                .map(|row| Movie {
                    id: row.get(0),
                    title: row.get(1),
                    year: row.get(2),
                    genre: row.get(3),
                    rating: row.get(4),
                })
                .collect();

            match serde_json::to_string(&movies) {
                Ok(json) => json,  // ✅ Convert Vec<Movie> to String
                Err(_) => "Failed to serialize movies".to_string(),
            }
        }
        Err(err) => {
            println!("Failed to fetch movies: {}", err);
            "[]".to_string() // ✅ Return empty JSON array on error
        }
    }
}
