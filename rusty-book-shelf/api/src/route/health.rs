use crate::handler::health::{health_check, health_check_db};
use axum::{routing::get, Router};
use registry::AppRegistry;

//↓RouterのStateがAppRegistryとなるため、Routerの型引数に指定する
pub fn build_health_check_routers() -> Router<AppRegistry> {
    let routers = Router::new() //healthcheckのパスのrootである/healthに個別のパスをネストする
        .route("/", get(health_check))
        .route("/db", get(health_check_db));
    Router::new().nest("/health", routers)
}
