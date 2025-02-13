use axum::{
    extract::{Request, State}, response::IntoResponse, Json
};
use bytes::Bytes;
use http_body_util::BodyExt;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, json};
use sqlx::{MySql, Pool, Row};
use chrono::{Utc, Duration};
use tower_cookies::{Cookie, Cookies};
#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct User {
    id: i32,
    username: String,
}

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id:i32,
    pub username:String,          // Subject (user identifier)
    pub exp: usize,           // Expiration time (Unix timestamp)
}

const SECRET: &str = "AxumandMiddleware";


pub async fn login(
    State(pool):State<Pool<MySql>>,
    cookies: Cookies,
    req: Request) -> String {
    let (parts, body) = req.into_parts();

    // Collect the body bytes
    let body_bytes: Bytes = match body.collect().await {
        Ok(full_body) => full_body.to_bytes(),
        Err(_) => {
            println!("Failed to collect body.");
            return  String::from("Failed to collect body");
        }
    };

    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Raw Body: {}", body_str);

    // Parse the body into a LoginRequest instance
    let curr_user: LoginRequest = match from_slice::<LoginRequest>(&body_bytes) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return String::from("Failed to parse JSON");
        }
    };

    // Print the constructed instance
    println!("Current User: {}", curr_user.username);

    let query = "Select id, username FROM users where username = ? AND password = ?";

    let user_result = sqlx::query(query)
    .bind(&curr_user.username) // Bind dynamic username
    .bind(&curr_user.password) // Bind dynamic password
    .fetch_optional(&pool) // Fetch a single optional result
    .await;

    println!("{:?}", user_result);

    let result = match user_result {
        Ok(Some(row)) => {
            let user = User {
                id: row.get(0),
                username: row.get(1),
            };
            println!("{:?}", user);
            serde_json::to_string(&user).unwrap_or_else(|_| "Failed to serialize user".to_string());
            user
        }
        Ok(None) => return "Invalid username or password".to_string(),
        Err(err) => return format!("Database error: {}", err),
    };

    let auth_token = create_jwt(&result);
    println!("{}", auth_token);
    

    


    // // Reconstruct the request with the consumed body
    // let new_body = axum::body::Body::from(body_bytes);
    // let _req = Request::from_parts(parts, new_body);

    let new_csrf_token = generate_csrf_token();
        println!("Generated new CSRF token: {}", new_csrf_token);

        // Update the CSRF token in the database
        if let Err(err) = sqlx::query("UPDATE users SET csrf_token = ? WHERE id = ?")
            .bind(&new_csrf_token)
            .bind(&result.id)
            .execute(&pool)
            .await
        {
            eprintln!("Failed to update CSRF token in database: {}", err);
        } else {
            println!("CSRF token updated in database for user ");
        }


            // Json(json!({
            //     "message": "Login successful, session created"
            // }))
            // .into_response()

            // let mut auth_cookie = Cookie::new("auth_token", auth_token.clone());
            // auth_cookie.set_path("/");
            // auth_cookie.set_secure(false);
            // auth_cookie.set_http_only(true);
            // auth_cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
         
            //  cookies.add(auth_cookie);
         
            //  let mut csrf_cookie = Cookie::new("auth_token", new_csrf_token.clone());
            // csrf_cookie.set_path("/");
            // csrf_cookie.set_secure(false);
            // csrf_cookie.set_http_only(true);
            // csrf_cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
         
            //  cookies.add(csrf_cookie);


            String::from("Login successful")





}

fn generate_csrf_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}




fn create_jwt(user: &User) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))  // Token expires in 60 minutes
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        id: user.id,
        username:(*user.username).to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
        .expect("JWT encoding failed")
}

// use axum::{
//     extract::{Request, State},
//     response::{IntoResponse, Json},
//     http::StatusCode,
// };
// use axum_extra::extract::cookie;
// use bytes::Bytes;
// use http_body_util::BodyExt;
// use rand::{distributions::Alphanumeric, Rng};
// use serde::{Serialize, Deserialize};
// use serde_json::{from_slice, json};
// use sqlx::{MySql, Pool, Row};
// use chrono::{Utc, Duration};
// use tower_cookies::{Cookie, Cookies};

// #[derive(Debug, Deserialize, Serialize)]
// struct LoginRequest {
//     username: String,
//     password: String,
// }

// #[derive(Serialize, Debug)]
// struct User {
//     id: i32,
//     username: String,
// }

// use jsonwebtoken::{encode, Header, EncodingKey};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Claims {
//     pub id: i32,
//     pub username: String,  
//     pub exp: usize,  
// }

// const SECRET: &str = "AxumandMiddleware";

// pub async fn login(
//     State(pool): State<Pool<MySql>>,
//     cookies: Cookies,
//     req: Request,
// ) -> impl IntoResponse {
//     let (parts, body) = req.into_parts();

//     // Collect the body bytes
//     let body_bytes: Bytes = match body.collect().await {
//         Ok(full_body) => full_body.to_bytes(),
//         Err(_) => {
//             return (StatusCode::BAD_REQUEST, "Failed to collect body").into_response();
//         }
//     };

//     // Parse the body into a LoginRequest instance
//     let curr_user: LoginRequest = match from_slice::<LoginRequest>(&body_bytes) {
//         Ok(parsed) => parsed,
//         Err(err) => {
//             return (StatusCode::BAD_REQUEST, format!("Failed to parse JSON: {}", err)).into_response();
//         }
//     };

//     // Fetch user from the database
//     let query = "SELECT id, username FROM users WHERE username = ? AND password = ?";

//     let user_result = sqlx::query(query)
//         .bind(&curr_user.username)
//         .bind(&curr_user.password)
//         .fetch_optional(&pool)
//         .await;

//     let user = match user_result {
//         Ok(Some(row)) => User {
//             id: row.get(0),
//             username: row.get(1),
//         },
//         Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response(),
//         Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)).into_response(),
//     };

//     let auth_token = create_jwt(&user);
//     let new_csrf_token = generate_csrf_token();

//     // Update CSRF token in the database
//     if let Err(err) = sqlx::query("UPDATE users SET csrf_token = ? WHERE id = ?")
//         .bind(&new_csrf_token)
//         .bind(&user.id)
//         .execute(&pool)
//         .await
//     {
//         return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to update CSRF token: {}", err)).into_response();
//     }

//     // Set secure cookies

    
    

//     // Return JSON response
//     Json(json!({
//         "message": "Login successful, session created"
//     }))
//     .into_response()
// }

// // Generates a random CSRF token
// fn generate_csrf_token() -> String {
//     rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(32)
//         .map(char::from)
//         .collect()
// }

// // Creates a JWT token
// fn create_jwt(user: &User) -> String {
//     let expiration = Utc::now()
//         .checked_add_signed(Duration::minutes(60))
//         .expect("valid timestamp")
//         .timestamp() as usize;

//     let claims = Claims {
//         id: user.id,
//         username: user.username.clone(),
//         exp: expiration,
//     };

//     encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
//         .expect("JWT encoding failed")
// }
