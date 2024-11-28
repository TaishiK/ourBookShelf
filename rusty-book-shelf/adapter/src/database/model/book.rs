use kernel::model::{
    book::Book,
    id::{BookId, UserId},
    user::BookOwner,
};


pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
    //pub publisher: String,
    //pub published_on: chrono::NaiveDate,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        //パターンマッチを用いて’BookRow’の中身を取り出す
        Self {
            id: value.book_id,
            title: value.title,
            author: value.author,
            isbn: value.isbn,
            description: value.description,
            owner: BookOwner {
                id: value.owned_by,
                name: value.owner_name,
            },
        }
    }
}
pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}
