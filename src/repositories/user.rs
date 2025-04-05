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
    async fn create(&self, _: UserInput) -> Result<User, RepositoryError> {
        Err(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Create is not implemented for user".to_string(),
        })
    }

    async fn read(&self, _: uuid::Uuid) -> Result<User, RepositoryError> {
        Err(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Read is not implemented for user".to_string(),
        })
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
