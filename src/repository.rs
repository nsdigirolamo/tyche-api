use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::dtos::post::CreatePostInput;
use crate::models::entities::Post;
use crate::models::errors::RepositoryError;

#[derive(Clone)]
pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub fn new(pool: Pool<Postgres>) -> Repository {
        Repository { pool }
    }

    pub async fn create_post(&self, input: CreatePostInput) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "INSERT INTO posts (author_id, message) VALUES ($1, $2) RETURNING id, author_id, message, created_at",
            input.author_id,
            input.message
        )
        .fetch_one(&self.pool)
        .await.map_err(|err| err.into())
    }

    pub async fn get_post(&self, id: Uuid) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "SELECT id, author_id, message, created_at FROM posts WHERE id = $1",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }
}
