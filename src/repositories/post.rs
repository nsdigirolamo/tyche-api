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
            "INSERT INTO posts (parent_id, author_id, body)
                VALUES ($1, $2, $3)
                RETURNING id, parent_id, author_id, body, created_at",
            input.parent_id,
            input.author_id,
            input.body
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    async fn read(&self, id: uuid::Uuid) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "SELECT id, parent_id, author_id, body, created_at FROM posts WHERE id = $1",
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
