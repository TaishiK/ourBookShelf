use crate::{extractor::AuthorizedUser, model::checkout::CheckoutResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::{
    checkout::event::{CreateCheckout, UpdateReturned},
    id::{BookId, CheckoutId}, user,
};
use registry::AppRegistry;
use shared::error::AppResult;

pub async fn checkout_book(
    user: AuthorizedUser,
    Path((book_id,)): Path<(BookId,)>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let create_checkout_history = 
        CreateCheckout::new(book_id, user.id, Utc::now());
    registry
        .checkout_repository()
        .create(create_checkout_history)
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn return_book(
    user: AuthorizedUser,
    Path((book_id, checkout_id)): Path<(BookId, CheckoutId)>,
    State(registry): State<AppRegistry>
) -> AppResult<StatusCode> {
    let update_returned = UpdateReturned::new(
        checkout_id, 
        book_id, 
        user.id, 
        chrono::Utc::now(),
    );
    registry
        .checkout_repository()
        .update_returned(update_returned)
        .await
        .map(|_| StatusCode::OK)
}
pub async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutResponse>> {
    registry
        .checkout_repository()
        .find_unreturned_all()
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
pub async fn checkout_history(
    _user: AuthorizedUser,
    Path((book_id,)): Path<(BookId,)>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .checkout_repository()
        .find_history_by_book_id(book_id)
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}