//use uuid::Uuid;
use crate::model::{id::BookId, user::BookOwner};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    //pub publisher: String,
    //pub published: i32,
    //price: i32,
}
#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}

