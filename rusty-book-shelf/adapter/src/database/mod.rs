use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

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
pub struct ConnectionPool(PgPool);//sqlx::PgPoolをラップした構造体
impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool { //sqlx::PgPoolへの参照を返すメソッド
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(&cfg)))
} //ConnectionPoolを作成し返す関数
