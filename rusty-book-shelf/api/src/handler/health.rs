use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;
//use utoipa::path;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check succeeded.")
    )
)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
#[utoipa::path(
    get,
    path = "/health/db",
    responses(
        (status = 200, description = "Database health check successful"),
        (status = 500, description = "Database health check failed")
    )
)]
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
