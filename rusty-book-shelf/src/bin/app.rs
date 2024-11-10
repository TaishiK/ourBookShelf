use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use std::net::{Ipv4Addr, SocketAddr};
//use api::handler::health::{health_check, health_check_db};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
//use axum::{ extract::State, http::StatusCode };
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?; //AppConfigの生成
    let pool = connect_database_with(&app_config.database); //データベース接続
    let registry = AppRegistry::new(pool); //AppRegistryの生成
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registry);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080); //サーバーの起動
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.map_err(Error::from)
}
/*
#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[sqlx::test]
async fn health_check_db_works(pool: sqlx::PgPool) {
    let status_code = health_check_db(State:registry).await;
    assert_eq!(status_code, StatusCode::OK)
} */
