use crate::handler::{
    book::{
        delete_book, register_book, show_book, show_book_list, update_book,
    },
    checkout::{
        checkout_book, checkout_history, return_book, show_checked_out_list,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use registry::AppRegistry;


pub fn build_book_routers() -> Router<AppRegistry> {
    let books_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", post(update_book))
        .route("/:book_id", post(delete_book));
    let checkout_routers = Router::new()
        .route("/checkouts", get(show_checked_out_list))
        .route("/:book_id/checkout", post(checkout_book))
        .route(
            "/:book_id/checkouts/:checkout_id/returned",
            post(return_book), //書籍ではputメソッドを使っているが、ここではpostメソッドを使っている
        )
        .route("/:book_id/checkout-history", get(checkout_history));
     
    Router::new().nest("/books", books_routers.merge(checkout_routers))
}
