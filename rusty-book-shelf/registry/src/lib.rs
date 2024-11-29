use adapter::{
    database::ConnectionPool, 
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl,
        book::BookRepositoryImpl,
        health::HealthCheckRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository,book::BookRepository,health::HealthCheckRepository,
};
use shared::config::AppConfig;
use std::sync::Arc;
use adapter::repository::user::UserRepositoryImpl;
use kernel::repository::user::UserRepository;

#[derive(Clone)] //↓ DIコンテナの役割を果たす構造体を定義（Dependency Injection）
pub struct AppRegistry {
    //cloneはAppRegistryをコピー可能にする　のちにaxum側で使う
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
    checkout_repository: Arc<dyn CheckoutRepository>,
}

impl AppRegistry {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        //関数内で手書きして依存解決を行う
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(), 
            redis_client.clone(), 
            app_config.auth.ttl,
        ));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
        let checkout_repository = 
            Arc::new(CheckoutRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            book_repository,
            auth_repository,
            user_repository,
            checkout_repository,
        }
    }
    //↓依存解決したインスタンスを返すメソッドを定義する
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }
    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
    pub fn checkout_repository(&self) -> Arc<dyn CheckoutRepository> {
        self.checkout_repository.clone()
    }
}
