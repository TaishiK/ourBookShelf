use crate::model::{
    checkout::{
        event::{ CreateCheckout, UpdateReturned },
        Checkout,
    },
    id::{ BookId, UserId },
};
use async_trait::async_trait;
use shared::error::AppResult;

#[async_trait]
pub trait CheckoutRepository: Send + Sync {
    async fn create(&self, event: CreateCheckout) -> AppResult<()>;//貸出操作
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()>;//返却操作
    async fn find_unreturned_all(&self) -> AppResult<Checkout>;//管理者用の未返却の貸出一覧
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>>;//ユーザーが借りている本の一覧
    async fn find_history_by_book_id(&self,book_id: BookId) -> AppResult<Vec<Checkout>>;//本の貸出履歴
}