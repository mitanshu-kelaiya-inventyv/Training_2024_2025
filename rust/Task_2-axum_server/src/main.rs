//axum server
use axum::{middleware, routing::get, Router};
use server::{root, get_foo, post_foo, foo_bar, timing_middleware}; //server is my current project name. Instead of crate:: i write server:: to import the functions from lib.rs
#[tokio::main]
async fn main()
{
    println!("Dependency downloaded");
    let app = Router::new()
    .route("/", get(root))
    .route("/foo",get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar))
    .layer(middleware::from_fn(timing_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



//without axum
// use std::{
//   net::{TcpListener, TcpStream},
//   io::{prelude::*, BufReader}
// };

// fn main() {
//   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//   for stream in listener.incoming() {
//       let stream = stream.unwrap();
//       handle_connection(stream);
//       //println!("Connection established!");
//   }
// }

// fn handle_connection(mut stream: TcpStream){
//   let buf_reader = BufReader::new(&stream);
//   let http_request: Vec<_> = buf_reader.lines()
//                                        .map(|result| result.unwrap())
//                                        .take_while(|line| !line.is_empty())
//                                        .collect();
//   println!("Request: {http_request:#?}");
//   let response = "HTTP/1.1 200 OK\r\n\r\n
//   <!DOCTYPE html>
// <html lang=\"en\">
// <head>
//   <meta charset=\"utf-8\">
//   <title>Hello!</title>
// </head>
// <body>
//   <h1>Hello!</h1>
//   <p>Hi from Rust</p>
// </body>
// </html>
// ";
//   stream.write_all(response.as_bytes()).unwrap();
// }