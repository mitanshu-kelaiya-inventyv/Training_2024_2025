use axum::{middleware::{self, from_fn_with_state}, routing::{delete, get, post}, Router};

use crate::{handlers, middlewares};
use sqlx::{MySql, Pool};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn get_routes(pool:&Pool<MySql>) -> Router<Pool<MySql>>{
    // Router::new()
    // .route("/", get(handlers::demo_handler::first_handler))
    // .route("/getMovies", get(handlers::get_movies::get_all))
    // .route("/getMovies/{year}", get(handlers::get_movies::get_by_year))
    // .layer(middleware::from_fn(middlewares::first::verify_token))


    // let public_routes = Router::new()
//     .route("/login", post(handlers::auth::login))
//     .route("/register", post(handlers::auth::register));

let user_routes = Router::new()
.route("/login", post(handlers::login::login))
.route("/register", post(handlers::register::register));

let movie_routes = Router::new()
.route("/get_movies", get(handlers::get_movies::get_all))
.route("/get_movies/{year}", get(handlers::get_movies::get_by_year))
.route("/add_movie", post(handlers::add_movie::add_movie))
.route("/delete_movie/{id}", delete(handlers::delete_movie::delete_movie))
.layer(from_fn_with_state(pool.clone(), middlewares::second::add_id))
.layer(middleware::from_fn(middlewares::first::read_headers));


    // Apply middleware only here
    Router::new()
    .nest("/user", user_routes)
    .nest("/movie", movie_routes)
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
    ).layer(axum::middleware::from_fn(middlewares::log::log_middleware_custom))
        .layer(TraceLayer::new_for_http())
    
    
    
    // .layer(middleware::from_fn(middlewares::first::read_body))
    // .layer(middleware::from_fn(middlewares::first::read_headers))



}