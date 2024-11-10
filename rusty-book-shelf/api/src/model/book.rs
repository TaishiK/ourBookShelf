use kernel::model::book::{event::CreateBook, Book};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)] //requestの値（Json形式）をRustの構造体に変換（deserialize)する
#[serde(rename_all = "camelCase")] //frontend側(Javascript)との連携のためにキャメルケースに変換
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, Serialize)] //responseの値（Rustの構造体）をJson形式に変換（serialize)する
#[serde(rename_all = "camelCase")] //frontend側(Javascript)との連携のためにキャメルケースに変換
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
