use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use std::net::{Ipv4Addr, SocketAddr};
use tracing::subscriber;
//use api::handler::health::{health_check, health_check_db};
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::Router;
//use axum::{ extract::State, http::StatusCode };
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
        .with(env_filter)
        .with(subscriber)
        .try_init()?;

    Ok(())
}
async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?; //AppConfigの生成
    let pool = connect_database_with(&app_config.database); //データベース接続
    let registry = AppRegistry::new(pool); //AppRegistryの生成
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .merge(vl::routes())
        .merge(auth::routes())
        .layer(cors())
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
    let listener = tokio::net::TcpListener::bind(&addr).await?;
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
