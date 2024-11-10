use adapter::repository::book::BookRepositoryImpl;
use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::book::BookRepository;
use kernel::repository::health::HealthCheckRepository;
use std::sync::Arc;

#[derive(Clone)] //↓ DIコンテナの役割を果たす構造体を定義（Dependency Injection）
pub struct AppRegistry {
    //cloneはAppRegistryをコピー可能にする　のちにaxum側で使う
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        //関数内で手書きして依存解決を行う
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
            book_repository,
        }
    }
    //↓依存解決したインスタンスを返すメソッドを定義する
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
}
