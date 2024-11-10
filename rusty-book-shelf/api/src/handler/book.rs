use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use registry::AppRegistry;
use thiserror::Error;
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};

#[derive(Error, Debug)]
pub enum AppError {
    //暫定のエラー処理記述
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
    }
}

pub async fn register_book(
    //書籍を登録するapi
    State(registry): State<AppRegistry>, //AppRegistryを取得
    Json(req): Json<CreateBookRequest>,  //リクエストボディを取得
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    //書籍一覧を取得するapi
    State(registry): State<AppRegistry>, //AppRegistryを取得
) -> Result<Json<Vec<BookResponse>>, AppError> {
    //レスポンスボディをJson形式で返す
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
        .map(Json)
        .map_err(AppError::from)
}

pub async fn show_book(
    //書籍詳細を取得するapi
    Path(book_id): Path<Uuid>,           //リクエストパラメータを取得
    State(registry): State<AppRegistry>, //AppRegistryを取得
) -> Result<Json<BookResponse>, AppError> {
    //レスポンスボディをJson形式で返す
    registry
        .book_repository()
        .find_by_id(book_id) //リクエストパラメータを元に書籍を取得
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(anyhow::anyhow!("The specified book was not found")),
        })
        .map_err(AppError::from)
}
