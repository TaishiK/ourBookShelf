//use anyhow::Result;//text p150 page付近AppResultに変更
use async_trait::async_trait;
use derive_new::new;
use kernel::model::{
    book::{event::CreateBook, Book},
    id::BookId,
};
use kernel::repository::book::BookRepository;

use shared::error::AppError;
use shared::error::AppResult;

use crate::database::model::book::BookRow;
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    //async fn create(&self, event: CreateBook) -> Result<()> {//anyhowをAppResultに変更
    async fn create(&self, event: CreateBook) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, author, isbn, description)
            VALUES ($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        //.await?;
        .await
        .map_err(AppError::SpecificOperationError)?;
        //sqlx::Error型をAppError型に変換
        Ok(())
    }

    //async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                WHERE book_id = $1
            "#,
            book_id as _ //query_as!マクロによるコンパイル時の型チェックを無効化
        )
        .fetch_optional(self.db.inner_ref())
        //.await?;
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from)) //()内削除かも？
    }
    //async fn find_all(&self) -> Result<Vec<Book>> {
    async fn find_all(&self) -> AppResult<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        //.await?;
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(rows.into_iter().map(Book::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool)); //BookRepositoryImplを初期化
        let book = CreateBook {
            //投入するための書籍データを作成
            title: "test Title".into(),
            author: "test Author".into(),
            isbn: "test ISBN".into(),
            description: "test Description".into(),
        };
        //書籍データを投入すると正常終了することを確認
        repo.create(book).await?;
        //書籍の一覧の取得をすると、投入した書籍データ１件が含まれていることを確認
        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        //書籍の一覧の最初のデータから書籍IDを取得し、そのIDで書籍データを取得すると、投入した書籍データが取得できることを確認
        let book_id = res[0].id;
        let res = repo.find_by_id(book_id.into()).await?;
        assert!(res.is_some());

        //取得した書籍データが投入した書籍データと一致することを確認
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "test Title");
        assert_eq!(author, "test Author");
        assert_eq!(isbn, "test ISBN");
        assert_eq!(description, "test Description");

        Ok(())
    }
}
