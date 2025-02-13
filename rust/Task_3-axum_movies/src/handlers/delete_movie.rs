use axum::extract::{Path, Request, State};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool, Row};

#[derive(Debug, Serialize, Deserialize)]
struct Movie{
    title:String,
    year:i32,
    genre: String,
    rating:i32,
}

pub async fn delete_movie( 
    Path(movie_id): Path<i32>,    
State(pool): State<Pool<MySql>>, 
req: Request  ) -> String{
    println!("Data deleted. {}", movie_id);

    if let Err(err) = sqlx::query("DELETE FROM movies WHERE id = ?")
    .bind(movie_id)
    .execute(&pool)
    .await
{
    println!("Database error: {}", err);
}



let query = "SELECT id, title, year, genre, rating FROM movies";
match sqlx::query(query).fetch_all(&pool).await {
    Ok(rows) => {
        let users: Vec<Movie> = rows.into_iter()
            .map(|row| Movie {
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

    // String::from(format!("Data deleted {}", id))
}