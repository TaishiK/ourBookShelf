use crate::database::ConnectionPool;
use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;

#[derive(new)] //Constructerを自動生成させる
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool, //構造体にConnectionPoolを持たせる
}

#[async_trait] //HealthCheckRepositoryを実装
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        //DB接続確立できればtrueを返すメソッド
        sqlx::query("SELECT 1") //DBに対してSELECT 1を実行 結果はResult型
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
