use crate::models::{dtos::post::PostInput, entities::Post, errors::RepositoryError};

#[derive(Clone)]
pub struct PostRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> PostRepository {
        PostRepository { pool }
    }

    pub async fn create_one(&self, input: PostInput) -> Result<Post, RepositoryError> {
        sqlx::query_as!(
            Post,
            "INSERT INTO posts (parent_id, author_id, body)
                VALUES ($1, $2, $3)
                RETURNING *",
            input.parent_id,
            input.author_id,
            input.body
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

    pub async fn find_one_by_id(&self, id: uuid::Uuid) -> Result<Post, RepositoryError> {
        sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_many_by_parent_id(
        &self,
        parent_id: Option<uuid::Uuid>,
    ) -> Result<Vec<Post>, RepositoryError> {
        match parent_id {
            Some(id) => sqlx::query_as!(Post, "SELECT * FROM posts WHERE parent_id = $1", id)
                .fetch_all(&self.pool)
                .await
                .map_err(|err| err.into()),
            None => sqlx::query_as!(Post, "SELECT * FROM posts WHERE parent_id IS NULL")
                .fetch_all(&self.pool)
                .await
                .map_err(|err| err.into()),
        }
    }
}
