use crate::models::{dtos::user::UserInput, entities::User, errors::RepositoryError};

use super::Repository;

#[allow(dead_code)]
#[derive(Clone)]
pub struct UserRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl UserRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> UserRepository {
        UserRepository { pool }
    }
}

impl Repository<User, UserInput> for UserRepository {
    async fn create(&self, input: UserInput) -> Result<User, RepositoryError> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password, created_at",
            input.username,
            input.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn read(&self, id: uuid::Uuid) -> Result<User, RepositoryError> {
        sqlx::query_as!(
            User,
            "SELECT id, username, password, created_at FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn update(&self, _: uuid::Uuid, _: UserInput) -> Result<User, RepositoryError> {
        Err(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Update is not implemented for user".to_string(),
        })
    }

    async fn delete(&self, _: uuid::Uuid) -> Option<RepositoryError> {
        Some(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Delete is not implemented for user".to_string(),
        })
    }
}
