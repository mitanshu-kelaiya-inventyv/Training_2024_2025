use axum::extract::Request;
use axum::middleware::Next;
use axum::{routing::get, Router};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{fs::OpenOptions, io::Write, sync::Mutex};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use chrono::Local;

use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_FILE: Mutex<std::fs::File> = Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("requests.log")
            .expect("Failed to open log file")
    );
}

pub async fn log_middleware_custom(request: Request, next: Next) -> impl axum::response::IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let start_time = Instant::now();
    
    let request_log_message = format!(
        "REQUEST - Method: {} - URI: {} - Time: {}\n",
        method, uri, request_time
    );
    
    if let Ok(mut file) = LOG_FILE.lock() {
        let _ = file.write_all(request_log_message.as_bytes());
    }
    
    let response = next.run(request).await;
    let duration = start_time.elapsed();
    let response_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let response_log_message = format!(
        "RESPONSE - Method: {} - URI: {} - Time: {} - Duration: {} ms\n", 
        method, uri, response_time, duration.as_millis()
    );
    
    if let Ok(mut file) = LOG_FILE.lock() {
        let _ = file.write_all(response_log_message.as_bytes());
    }
    
    response
}





// use axum::{routing::get, Router};
// use std::net::SocketAddr;
// use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};
// use tracing::{info, Level};
// use tracing_subscriber::{fmt, EnvFilter};

// async fn handler() -> &'static str {
//     "Hello, World!"
// }

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::new("info"))
//         .init();
    
//     let app = Router::new()
//         .route("/", get(handler))
//         .layer(
//             TraceLayer::new_for_http()
//                 .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
//                 .on_request(DefaultOnRequest::new().level(Level::INFO))
//                 .on_response(DefaultOnResponse::new().level(Level::INFO))
//         );
    
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     info!("Listening on {}", addr);
    
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

