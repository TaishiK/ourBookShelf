use anyhow::Result;

pub struct AppConfig {
    //アプリケーションの設定を保持する構造体
    pub database: DatabaseConfig,
}
impl AppConfig {
    //データベース接続に必要な情報を環境変数から取り出す処理
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };
        Ok(Self { database })
    }
}

pub struct DatabaseConfig {
    //DB接続設定を保持する構造体
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct RedisConfig {
    //Redis接続設定を保持する構造体
    pub host: String,
    pub port: u16,
}

