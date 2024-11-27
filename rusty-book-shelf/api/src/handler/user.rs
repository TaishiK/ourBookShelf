use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
//use garde::Validate;
use kernel::model::{id::UserId, user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};
use crate::{
    extractor::AuthorizedUser,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest,
        UpdateUserPasswordRequestWithUserId, UpdateUserRoleRequest,
        UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};
pub async fn register_user(//This function is used to register a user
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    if !user.is_admin() {//Admin can only register a user
        return Err(AppError::ForbiddenOperation);
    }
    let registered_user =
        registry.user_repository().create(req.into()).await?;
    Ok(Json(registered_user.into()))
}

pub async fn list_users(//This function is used to list all users
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UsersResponse>> {
    let items = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();
    Ok(Json(UsersResponse { items }))
}

pub async fn delete_user(//This function is used to delete a user
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {//Admin can only delete a user
        return Err(AppError::ForbiddenOperation);
    }
    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;
    Ok(StatusCode::OK)
}

pub async fn change_role(//This function is used to change the role of a user
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {//Admin can only change the role of a user
        return Err(AppError::ForbiddenOperation);
    }
    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithUserId::new(user_id, req).into())
        .await?;
    Ok(StatusCode::OK)
}
pub async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    registry
        .user_repository()
        .update_password(
            UpdateUserPasswordRequestWithUserId::new(user.id(), req).into(),
        )
        .await?;
    Ok(StatusCode::OK)
}
pub async fn get_current_user(//This function is used to get the current user
    user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}