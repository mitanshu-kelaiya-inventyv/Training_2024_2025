use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
    body::Body,
    Extension,
};
use bytes::Bytes;
use http_body_util::BodyExt;
use serde::{Serialize, Deserialize};
use serde_json::from_slice;

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

use crate::handlers::login::Claims;

const SECRET: &str = "AxumandMiddleware";

pub async fn read_headers(mut req: Request, next: Next) -> Response {
    let headers: &HeaderMap = req.headers();
        
    // for (key, value) in headers.iter() {
    //     println!("Header: {} = {:?}", key, value);
    // }
    if let Some(token) = headers.get("auth_token").and_then(|v| v.to_str().ok()) {
        println!("Authorization header is present");

        match verify_jwt(token) {
            Ok(claims) => {
                println!("Token is valid, user ID: {}", claims.id);

                // Insert `user_id` into headers for tracking
                // let mut headers_mut = req.headers_mut();
                // headers_mut.insert(
                //     "user_id",
                //     HeaderValue::from_str(&claims.id.to_string()).unwrap(),
                // );

                // Store claims directly inside request extensions (no Arc/Mutex needed)
                req.extensions_mut().insert(claims.clone());
                if let Some(inserted_claims) = req.extensions().get::<Claims>() {
                    println!("Inserted claims in first: {:?}", inserted_claims);
                } else {
                    println!("No claims found in request extensions after insert!");
                }
                println!("{:?}",req);
                // Continue processing the request
                next.run(req).await
            }
            Err(_) => Json("Invalid token").into_response(),
        }
     } else {
        Json("No token found").into_response()
    }
    
    
}

pub async fn read_body(req: Request, next: Next) -> Response {
    let (parts, body) = req.into_parts();

    // Collect the body bytes properly
    let body_bytes: Bytes = match body.collect().await {
        Ok(full_body) => full_body.to_bytes(),
        Err(_) => Bytes::new(),
    };

    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("Raw Body: {}", body_str);
    let mut curr_user:LoginRequest;
    // Parse the body as JSON
    match from_slice::<LoginRequest>(&body_bytes) {
        Ok(parsed) => {
            println!("Parsed Login Request: {:?}", parsed);
            // curr_user = parsed;
            // println!("{}{}", parsed.username, parsed.password);
        }
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
        }
    }
    // Reconstruct the request with the consumed body
    let new_body = Body::from(body_bytes);
    let req = Request::from_parts(parts, new_body);

    next.run(req).await
}




fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    println!("{:?}", token_data.claims.id);
    Ok(token_data.claims)
}