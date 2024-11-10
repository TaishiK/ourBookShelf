use crate::model::book::{event::CreateBook, Book};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Book>>;
    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
    async fn create(&self, event: CreateBook) -> Result<()>;
    //async fn update(&self, book: Book) -> Result<Book>;
    //async fn delete(&self, id: Uuid) -> Result<()>;
}
