use crate::models::{dtos::post::PostInput, entities::Post, errors::RepositoryError};

use super::Repository;

#[derive(Clone)]
pub struct PostRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> PostRepository {
        PostRepository { pool }
    }
}

impl Repository<Post, PostInput> for PostRepository {
    async fn create(&self, input: PostInput) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "INSERT INTO posts (author_id, message) VALUES ($1, $2) RETURNING id, author_id, message, created_at",
            input.author_id,
            input.message
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn read(&self, id: uuid::Uuid) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "SELECT id, author_id, message, created_at FROM posts WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn update(&self, _: uuid::Uuid, _: PostInput) -> Result<Post, RepositoryError> {
        Err(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Update is not implemented for post".to_string(),
        })
    }

    async fn delete(&self, _: uuid::Uuid) -> Option<RepositoryError> {
        Some(RepositoryError {
            status: warp::http::StatusCode::NOT_IMPLEMENTED,
            message: "Delete is not implemented for post".to_string(),
        })
    }
}
