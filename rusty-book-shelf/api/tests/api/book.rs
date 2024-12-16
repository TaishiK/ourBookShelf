use std::sync::Arc;
use axum::{body::Body, http::Request};
use rstest::rstest;
use tower::ServiceExt;

use crate::{
    deserialize_json,
    helper::{fixture, make_router, v1, TestRequestExt},
};
use api::model::book::PaginatedBookResponse;
use kernel::{
    model::{
        book::Book,
        id::{BookId, UserId},
        list::PaginatedList,
        user:: BookOwner,
    },
    repository::book::MockBookRepository,
};

#[rstest]
#[case("/books", 20, 0)]
#[case("/books?limit=50", 50, 0)]
#[case("/books?limit=50&offset=20", 50, 20)]
#[case("/books?offset=20", 20, 20)]
#[tokio::test]
async fn show_book_list_with_query_200 (//rstestでパラメータ化テストを行う
    mut fixture: registry:: MockAppRegistryExt,//fixtureを受け取る
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
) -> anyhow::Result<()> {
    let book_id = BookId::new();

    // モックの振る舞いを設定
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "RustによるWebアプリケーション開発".to_string(),
                author: "Rust太郎".to_string(),
                isbn: "".to_string(),
                description: "RustでWebアプリケーションを開発するための本です".to_string(),
                owner: BookOwner{
                    id: UserId::new(),
                    name: "Rust太郎".to_string(),
            },
            checkout: None,
        }];
        Ok(PaginatedList {
            total: 1,
            limit: opt.limit,
            offset: opt.offset,
            items,
        })
    });
    Arc::new(mock)
});
    // テスト対象のルーターを作成
    let app: axum::Router = make_router(fixture);//fixtureを使ってrouterを作成
    // テスト対象のリクエストを作成・送信し、レスポンスのステータスコードを検証する
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let resp = app.oneshot(req).await?;
    println!("status: {}", resp.status());
    assert_eq!(resp.status(), axum::http::StatusCode::OK);
    // レスポンスボディをデシリアライズして検証する
    let result = deserialize_json!(resp, PaginatedBookResponse);
    println!("title: {}", result.items[0].title);
    println!("author: {}", result.items[0].author);
    println!("description: {}", result.items[0].description);
    assert_eq!(result.limit, expected_limit);//limitが期待通りか検証
    assert_eq!(result.offset, expected_offset);//offsetが期待通りか検証
    //testが成功していることを示す
    Ok(())

}
