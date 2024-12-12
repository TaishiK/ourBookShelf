use async_trait::async_trait;

#[mockall::automock]
#[async_trait] //async-traitマクロを使用
pub trait HealthCheckRepository: Send + Sync {
    //Send と Sync はマーカートレイト
    async fn check_db(&self) -> bool; //DB接続確立できればtrueを返すメソッド
}
