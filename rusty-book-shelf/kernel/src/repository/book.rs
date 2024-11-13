use crate::model::{
    book::{event::CreateBook, Book},
    id::BookId,
};
//use anyhow::Result;
use async_trait::async_trait;
use shared::error::AppResult; //anyhowをAppResultに変更

#[async_trait]
pub trait BookRepository: Send + Sync {
    //async fn find_all(&self) -> Result<Vec<Book>>;//anyhowをAppResultに変更
    async fn find_all(&self) -> AppResult<Vec<Book>>;
    //async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>>;
    //async fn create(&self, event: CreateBook) -> Result<()>;
    async fn create(&self, event: CreateBook) -> AppResult<()>;
    //async fn update(&self, book: Book) -> Result<Book>;
    //async fn delete(&self, id: Uuid) -> Result<()>;
}
