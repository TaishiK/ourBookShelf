use async_trait::async_trait;
use shared::error::AppResult;
use crate::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<UserId>;//書籍の内容からOptionを削除
    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId>;
    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken>;
    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()>;
}