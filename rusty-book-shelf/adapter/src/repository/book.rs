//use anyhow::Result;//text p150 page付近AppResultに変更
use async_trait::async_trait;
use derive_new::new;
use kernel::model::{
    id::{BookId, UserId},
    {book::event::DeleteBook, list::PaginatedList},
};
use kernel::{
    model::book::{
        event::{CreateBook, UpdateBook},
        Book, BookListOptions,
    },
    repository::book::BookRepository,
};

use shared::error::AppError;
use shared::error::AppResult;

use crate::database::model::book::{BookRow, PaginatedBookRow};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, user_id:UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, author, isbn, description, user_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn find_all(
        &self,
        options: BookListOptions,
    ) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;
        let rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT
                    COUNT(*) OVER() AS "total!",
                        b.book_id AS "id: BookId"
                FROM books AS b
                ORDER BY b.created_at DESC
                LIMIT $1 
                OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let total = rows.first().map(|r| r.total).unwrap_or_default();
        let book_ids = rows.into_iter().map(|r| r.id).collect::<Vec<BookId>>();

        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id AS "book_id: BookId",
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS "owned_by: UserId",
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id = ANY($1::uuid[])
                ORDER BY b.created_at DESC
            "#,
            &book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        
        let items = rows.into_iter().map(Book::from).collect();

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })

    }
    
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id as "book_id: BookId",
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS "owned_by: UserId",
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id = $1
            "#,
            book_id as _ //query_as!マクロによるコンパイル時の型チェックを無効化
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from)) //()内削除かも？
    }

    async fn update(&self, event: UpdateBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET 
                    title = $1, 
                    author = $2, 
                    isbn = $3, 
                    description = $4
                WHERE book_id = $5
                AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() == 0 {
            return Err(AppError::EntityNotFound(
                "specified book not found".into(),));
        }
        Ok(())
    }

    async fn delete(&self, event: DeleteBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1
                AND user_id = $2
            "#,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "specified book not found".into(),));
        }
        Ok(())
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::UserRepositoryImpl;
    use kernel::{
        model::user::event::CreateUser,
        repository::user::UserRepository,
    };

    //#[ignore] //テストを(一時的に）無視するためのアトリビュート
    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        sqlx::query!(r#"INSERT INTO roles(name) VALUES('Admin'), ('User');"#)
            .execute(&pool)
            .await?;
        let user_repo = 
            UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool.clone())); //BookRepositoryImplを初期化
        let user =user_repo
            .create(CreateUser {
                name: "test user".into(),
                email: "test@example.com".into(),
                password: "test_password".into(),
            })
            .await?;

        let book = CreateBook {
            //投入するための書籍データを作成
            title: "test Title".into(),
            author: "test Author".into(),
            isbn: "test ISBN".into(),
            description: "test Description".into(),
        };
        //書籍データを投入すると正常終了することを確認
        repo.create(book, user.id).await?;
        //書籍の一覧の取得をすると、投入した書籍データ１件が含まれていることを確認
        let options = BookListOptions { limit: 20, offset: 0 };
        let res = repo.find_all(options).await?;
        assert_eq!(res.items.len(), 1);

        //書籍の一覧の最初のデータから書籍IDを取得し、そのIDで書籍データを取得すると、投入した書籍データが取得できることを確認
        let book_id = res.items[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        //取得した書籍データが投入した書籍データと一致することを確認
        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "test Title");
        assert_eq!(author, "test Author");
        assert_eq!(isbn, "test ISBN");
        assert_eq!(description, "test Description");
        assert_eq!(owner.name, "test User");

        Ok(())
    }
}
