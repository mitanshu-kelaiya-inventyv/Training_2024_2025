use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::Response,
};
use std::time::Instant;
use tokio::time::sleep;
use std::time::Duration;

// // Middleware function to measure request processing time
pub async fn timing_middleware(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now(); // Start timer
    let response = next.run(req).await; // Call next middleware/handler
    let duration = start.elapsed(); // Calculate elapsed time

    println!("Request took: {:?}", duration);

    response
}

// // Example request handler
// async fn handle_request() -> &'static str {
//     // Simulate some processing time
//     sleep(Duration::from_millis(500)).await;
//     "Hello, Axum!"
// }

// #[tokio::main]
// async fn main() {
//     // Build the Axum app with middleware
//     let app = Router::new()
//         .route("/", axum::routing::get(handle_request))
//         .layer(middleware::from_fn(timing_middleware)); // Apply timing middleware

//     let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
//     println!("Server running on http://{}", addr);

//     // Run the server
//     Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
// }

pub async fn root() -> &'static str {
    "Root"
}

pub async fn get_foo() -> &'static str {
    "Get_Foo"
}

pub async fn post_foo() -> &'static str {
    sleep(Duration::from_secs(4)).await;
    "Post_Foo"
}

pub async fn foo_bar() -> &'static str {
    "Foo_Bar"
}
