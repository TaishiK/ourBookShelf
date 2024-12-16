use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Error, Result};
use std::{net::{Ipv4Addr, SocketAddr}, sync::Arc};
use api::route::{auth, v1};
use axum::{http::Method, Router};
use anyhow::Context;
//use registry::AppRegistry;
use registry::AppRegistryImpl;
use shared::config::AppConfig;
use shared::env::{which, Environment};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tower_http::cors::{self, CorsLayer};
use opentelemetry::global;
use opentelemetry_jaeger::propagator::JaegerPropagator;


fn cors() -> CorsLayer {//CORSの設定-フロントエンドとの通信を許可
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT
        ])
        .allow_origin(cors::Any)
}

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
    //環境変数の読み込み
    let host = std::env::var("JAEGER_HOST")?;
    let port = std::env::var("JAEGER_PORT")?;
    let endpoint = format!("http://{host}:{port}/api/traces");

    global::set_text_map_propagator(JaegerPropagator::new());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_collector_endpoint(endpoint)
        .with_service_name("book-shelf")
        .with_auto_split_batch(true)
        .with_max_packet_size(8192)
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);


    let env_filter = //ログレベルを設定
        EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer() //ログの出力形式を設定
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    #[cfg(not(debug_assertions))]//デバッグモードでない場合（＝リリースビルド）
    let subscriber = subscriber.json();//本番環境（リリースビルド）ではjson形式でログが出力される

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?; //AppConfigの生成
    let pool = connect_database_with(&app_config.database); //データベース接続
    let kv = Arc::new(RedisClient::new(&app_config.redis)?); //Redis接続
    //let registry = AppRegistry::new(pool, kv, app_config); //AppRegistryの生成
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config)); //AppRegistryの生成
    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        .layer(cors())//CORSの設定-フロントエンドとの通信を許可
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response (
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
