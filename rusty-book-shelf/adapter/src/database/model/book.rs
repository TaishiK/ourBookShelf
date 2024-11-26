use kernel::model::book::Book;
//use kernel::model::id::BookId;
use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
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
        }
    }
}
