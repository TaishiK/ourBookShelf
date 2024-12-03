use kernel::model::{
    book::{Book, Checkout},
    id::{BookId, CheckoutId, UserId},
    user::{BookOwner, CheckoutUser},
};
use chrono::{DateTime, Utc};

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

impl BookRow {
    pub fn into_book(self, checkout: Option<Checkout>) -> Book {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owned_by,
            owner_name,
        } = self;
        Book {
            id: book_id,
            title,
            author,
            isbn,
            description,
            owner: BookOwner {
                id: owned_by,
                name: owner_name,
            },
            checkout,
        }
    }
}

/*impl From<BookRow> for Book {
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
}*/
pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}
pub struct BookCheckoutRow {//貸出情報を格納するBookCheckOutRow型
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub user_name: String,
    pub checked_out_at: DateTime<Utc>,
}

impl From<BookCheckoutRow> for Checkout {//CheckoutRow型入力をCheckout型に変換するFromトレイトの実装
    fn from(value: BookCheckoutRow) -> Self {
        let BookCheckoutRow {
            checkout_id,
            book_id: _, // この書き方が分からない。？？
            user_id,
            user_name,
            checked_out_at,
        } = value;
        Checkout {
            checkout_id,
            checked_out_by: CheckoutUser {
                id: user_id,
                name: user_name,
            },
            checked_out_at,
        }
    }
}