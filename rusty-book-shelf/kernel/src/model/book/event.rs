use crate::model::id::{BookId, UserId};

pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    //pub publisher: String,
    //pub published_on: chrono::NaiveDate,
    //price: i32,
}

#[derive(Debug)]
pub struct UpdateBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub requested_user: UserId,
}
#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
    pub requested_user: UserId,
}