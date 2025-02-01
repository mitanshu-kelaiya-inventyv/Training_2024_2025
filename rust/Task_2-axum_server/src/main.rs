use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        //println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader.lines()
                                         .map(|result| result.unwrap())
                                         .take_while(|line| !line.is_empty())
                                         .collect();
    println!("Request: {http_request:#?}");
    let response = "HTTP/1.1 200 OK\r\n\r\n
    <!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
";
    stream.write_all(response.as_bytes()).unwrap();
}


//axum server
// use axum::{routing::get, Router};
// #[tokio::main]
// async fn main()
// {
//     println!("Dependency downloaded");
//     let app = Router::new()
//     .route("/", get( get(root)))
//     .route("/foo",get(get_foo).post(post_foo))
//     .route("/foo/bar", get(foo_bar));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// async fn root()-> &'static str{
//     "Root"
// }

// async fn get_foo()-> &'static str{
//     "Get_Foo"
// }

// async fn post_foo()-> &'static str{
//     "Post_Foo"
// }

// async fn foo_bar()-> &'static str{
//     "Foo_Bar"
// }
