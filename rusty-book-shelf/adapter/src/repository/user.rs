use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use kernel::model::id::UserId;
use kernel::model::user::{
    event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
    User,
};
use kernel::repository::user::UserRepository;
use shared::error::{AppError, AppResult};
use crate::database::{model::user::UserRow, ConnectionPool};
//use sqlx::types::Uuid;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;


#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

impl UserRepositoryImpl {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            //.connect_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;
        Ok(Self{ db })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                u.user_id as "user_id: Uuid",
                u.name,
                u.email,
                r.name as role_name,
                u.created_at,
                u.updated_at
                FROM users AS u
                INNER JOIN roles AS r USING(role_id)
                WHERE u.user_id = $1
            "#,
            current_user_id.0
        )
        .fetch_optional(self.db)
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(row.map(|user_row| User{
            id:user_row.user_id,
            name: user_row.name,
            email:user_row.email,
            role: user_row.role_name,
            created_at: user_row.created_at,
            updated_at: user_row.updated_at,
        }))
        
        
        /*match row {
            Some(r) => Ok(Some(User::try_from(r)?)),
            None => Ok(None),
        }*/
    }

   
    async fn find_all(&self) -> AppResult<Vec<User>> {
        todo!()
    }
    async fn create(&self, _event: CreateUser,) -> AppResult<User> {
        todo!()
    }
    async fn update_password(&self, _event: UpdateUserPassword,) -> AppResult<()> {
        todo!()
    }
    async fn update_role(&self, _event: UpdateUserRole) -> AppResult<()> {
        todo!()
    }
    async fn delete(&self, _event: DeleteUser,) -> AppResult<()> {
        todo!()
    }

}