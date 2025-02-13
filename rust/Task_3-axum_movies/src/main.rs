use std::f64::consts::E;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};
use axum_movies::{routes, database_connection, AppState};
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Row, Pool};

#[tokio::main]
async fn main() {

    // let file_appender = rolling::daily(".", "requests.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

      
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::new("info"))
    //     .with_writer(non_blocking)
    //     .init();

    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::new("info"))
    .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let pool_state = match database_connection().await{
        Ok(pool) => pool,
        Err(err)=>{
            eprintln!("Could not connect to database: {}", err);
            return;
        }
    };
    let app = routes::get_routes(&pool_state).with_state(pool_state);
    axum::serve(listener, app).await.unwrap();
    
}
