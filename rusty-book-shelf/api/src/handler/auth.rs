use crate::{
    extractor::AuthorizedUser,
    model::auth::{AccessTokenResponse, LoginRequest},
};
use axum::{extract::State, http::StatusCode, Json};
use kernel::model::auth::event::CreateToken;
use registry::AppRegistry;
use shared::error::AppResult;

#[utoipa::path(
    get,
    path = "/auth/login",
    responses(
        (status = 200, description = "A user logged in"),
        (status = 500, description = "Failed to login")
    )
)]
pub async fn login(
    State(registry): State<AppRegistry>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let user_id = registry
        .auth_repository()
        .verify_user(&req.email, &req.password)
        .await?;
    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;
    Ok(Json(AccessTokenResponse {
        user_id,
        access_token: access_token.0,
    }))
    }

#[utoipa::path(
    get,
    path = "/auth/logout",
    responses(
        (status = 200, description = "A user logged out"),
        (status = 500, description = "Failed to logout")
    )
)]
pub async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .auth_repository()
        .delete_token(user.access_token)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

