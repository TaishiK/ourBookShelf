use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

/*async fn hello_world() -> &'static str {//handler function
    "Hello, Rust World!\n"
}*/
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result <() > {//非同期化されたmain関数
    //let app = Router::new().route("/hello",get(hello_world));
    let app = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
