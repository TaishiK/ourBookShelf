use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,// Default value
    Production, // Production value
}

pub fn which() -> Environment {//開発環墫か本番環境かを判定
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;//
    match env::var("ENV") {
        Ok(v) => v.parse().unwrap_or(default_env),
        Err(_) => default_env,
    }
}