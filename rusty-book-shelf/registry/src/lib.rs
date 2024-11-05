use std::sync::Arc;
sue adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

#[derive(clone)] //↓ DIコンテナの役割を果たす構造体を定義（Dependency Injection）
pub struct AppRegistry { //cloneはAppRegistryをコピー可能にする　のちにaxum側で使う
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {　//関数内で手書きして依存解決を行う
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        Self { health_check_repository, }
    }
    //↓依存解決したインスタンスを返すメソッドを定義する
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
