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

impl Repository<User, UserInput, String> for UserRepository {
    async fn create(&self, input: UserInput) -> Result<User, RepositoryError> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, password) VALUES ($1, $2) RETURNING *",
            input.name,
            input.password,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn read(&self, primary_key: String) -> Result<User, RepositoryError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE name = $1", primary_key)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into())
    }

    async fn update(&self, _: String, _: UserInput) -> Result<User, RepositoryError> {
        Err(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Update is not implemented for user".to_string(),
        })
    }

    async fn delete(&self, _: String) -> Option<RepositoryError> {
        Some(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Delete is not implemented for user".to_string(),
        })
    }
}
