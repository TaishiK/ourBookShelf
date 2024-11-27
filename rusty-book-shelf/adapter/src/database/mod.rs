use shared::{
    config::DatabaseConfig,
    error::{AppError, AppResult},
};
use sqlx::{postgres::PgConnectOptions, PgPool};

pub mod model;

//DatabaseConfigからPgConnectOptionsに変換する関数
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool); //sqlx::PgPoolをラップした構造体
impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        //sqlx::PgPoolへの参照を返すメソッド
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg))) // cfgの前から＆削除10Nov2024
} //ConnectionPoolを作成し返す関数

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
    pub async fn begin(
        &self,
    ) -> AppResult<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.0.begin().await.map_err(|e|AppError::TransactionError(e))
    }
}
