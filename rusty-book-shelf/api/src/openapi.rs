use crate::{ handler, model };

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Book App -書籍「RustによるWebアプリケーション開発」のサンプルアプリケーション",
        contact (
            name = "RustによるWebアプリケーション開発",
            url = "todo",
            email = "todo"
        ),
        description = r#"(書籍「RustによるWebアプリケーション開発」のサンプルアプリケーションです。)"#,
    ),
    paths(
        handler::health::health_check,
        handler::health::health_check_db,
        handler::book::show_book_list,
        handler::book::show_book,
        handler::book::register_book,
        handler::book::update_book,
        handler::book::delete_book,
        handler::checkout::checkout_book,
        handler::checkout::return_book,
        handler::checkout::checkout_history,
        handler::user::get_current_user,
        handler::auth::login,
        handler::auth::logout,
    ),
    components(schemas(
            model::book::CreateBookRequest,
            model::book::UpdateBookRequest,
            model::book::BookResponse,
            model::book::PaginatedBookResponse,
            model::book::BookCheckoutResponse,
            model::checkout::CheckoutsResponse,
            model::checkout::CheckoutResponse,
            model::checkout::CheckoutBookResponse,
            model::user::BookOwner,
            model::user::CheckoutUser,
            model::auth::LoginRequest,
            model::auth::AccessTokenResponse
        ))
    )]

pub struct ApiDoc;