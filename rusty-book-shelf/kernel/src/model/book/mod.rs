use uuid::Uuid;
pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    //pub publisher: String,
    //pub published: i32,
    //price: i32,
}
