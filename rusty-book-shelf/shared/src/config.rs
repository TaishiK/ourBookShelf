use anyhow::Result;

pub struct AppConfig {
    //アプリケーションの設定を保持する構造体
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
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
        let redis = RedisConfig {
                host: std::env::var("REDIS_HOST")?,
                port: std::env::var("REDIS_PORT")?.parse::<u16>()?,
        };
        let auth = AuthConfig {
            ttl: std::env::var("AUTH_TOKEN_TTL")?.parse::<u64>()?,
        };
        Ok(Self { database, redis, auth, })
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
pub struct AuthConfig {
    //認証トークンの有効期限を保持する構造体
    pub ttl: u64,
}

