#[cfg(test)]
mod tests {
    #[sqlx::test(fixtures("common"))]
    async fn it_works(pool: sqlx::PgPool) {
        let row = sqlx::query!("SELECT author FROM books WHERE title like '1984'")
            .fetch_one(&pool)
            .await
            .unwrap();  // This is a blocking operation
        let result = row.author;
        assert_eq!(result, "George Orwell".to_string());
    }
}
