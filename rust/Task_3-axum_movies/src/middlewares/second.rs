use axum::{extract::{Request, State}, http::{HeaderMap, HeaderValue, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json};
use axum_extra::headers;
use rand::{Rng, thread_rng, distributions::Alphanumeric};

use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool, Row};

use crate::handlers::login::Claims;
use crate::handlers::register::User;
use tower_cookies::{Cookie, Cookies}; 

pub async fn add_id(State(pool): State<Pool<MySql>>,
mut req: Request, next: Next) -> Response {
    let headers: &HeaderMap = req.headers();
    // Clone claims to avoid immutable borrow
    let claims = req.extensions().get::<Claims>().cloned();
    let claims_2 = req.extensions().get::<Claims>().cloned();

    if let Some(claims) = claims {
        println!("Verifying csrf token");
        if let Some(token) = headers.get("csrf_token").and_then(|v| v.to_str().ok()) {
        let result = match sqlx::query("SELECT csrf_token FROM users WHERE id = ?")
        .bind(&claims.id)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => {
            row.get::<String, _>("csrf_token")
        },
        Err(err) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        ).into_response(),
    }; 
    println!("{:?}", result);
        if result == token{
                println!("VALID");
            }else{
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid Access:"),
                ).into_response();
            }
        }
        

        println!("Adding user ID {} to headers", claims.id);

        // Modify headers to add the user ID
        let headers_mut = req.headers_mut();
        headers_mut.insert(
            "user_id",
            HeaderValue::from_str(&claims.id.to_string()).unwrap(),
        );
    } else {
        println!("No claims found in request extensions");
    }

    let response =next.run(req).await;

        let new_csrf_token = generate_csrf_token();
        println!("Generated new CSRF token: {}", new_csrf_token);

        // Update the CSRF token in the database
        if let Err(err) = sqlx::query("UPDATE users SET csrf_token = ? WHERE id = ?")
            .bind(&new_csrf_token)
            .bind(&claims_2.unwrap().id)
            .execute(&pool)
            .await
        {
            eprintln!("Failed to update CSRF token in database: {}", err);
        } else {
            println!("CSRF token updated in database for user ");
        }
        

        // Add new CSRF token to response headers
        // response.headers_mut().insert(
        //     "new_csrf_token",
        //     HeaderValue::from_str(&new_csrf_token).unwrap(),
        // );


    response


}



fn generate_csrf_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
