use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Error, Result};
use std::{net::{Ipv4Addr, SocketAddr}, sync::Arc};
use api::route::{auth, v1};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use anyhow::Context;
use registry::AppRegistry;
use shared::config::AppConfig;
use shared::env::{which, Environment};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use api::handler::book::{
    delete_book, register_book, show_book, show_book_list, update_book,
};


#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?; //ログの初期化
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };
    let env_filter = //ログレベルを設定
        EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer() //ログの出力形式を設定
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

pub fn build_book_routers() -> Router<AppRegistry> {
    let books_routers = Router::new()
        .route("/", get(show_book_list))
        .route("/", post(register_book))
        .route("/:book_id", get(show_book))
        .route("/:book_id", put(update_book))
        .route("/:book_id", delete(delete_book));
    Router::new().nest("/books", books_routers)
}
async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?; //AppConfigの生成
    let pool = connect_database_with(&app_config.database); //データベース接続
    let kv = Arc::new(RedisClient::new(&app_config.redis)?); //Redis接続
    let registry = AppRegistry::new(pool, kv, app_config); //AppRegistryの生成
    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080); //サーバーの起動
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in the server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
        .map_err(Error::from)
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
