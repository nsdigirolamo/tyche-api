use crate::models::{dtos::post::PostInput, entities::Post, errors::RepositoryError};

#[derive(rocket_db_pools::Database)]
#[database("post_repository")]
pub struct PostRepository(rocket_db_pools::sqlx::PgPool);

pub async fn create_one(
    mut db: rocket_db_pools::Connection<PostRepository>,
    input: PostInput,
) -> Result<Post, RepositoryError> {
    sqlx::query_as!(
        Post,
        "INSERT INTO posts (parent_id, author_id, body)
                VALUES ($1, $2, $3)
                RETURNING *",
        input.parent_id,
        input.author_id,
        input.body
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|err| err.into())
}

pub async fn find_one_by_id(
    mut db: rocket_db_pools::Connection<PostRepository>,
    id: uuid::Uuid,
) -> Result<Post, RepositoryError> {
    sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| err.into())
}

pub async fn find_many_by_parent_id(
    mut db: rocket_db_pools::Connection<PostRepository>,
    parent_id: Option<uuid::Uuid>,
) -> Result<Vec<Post>, RepositoryError> {
    match parent_id {
        Some(id) => sqlx::query_as!(Post, "SELECT * FROM posts WHERE parent_id = $1", id)
            .fetch_all(&mut **db)
            .await
            .map_err(|err| err.into()),
        None => sqlx::query_as!(Post, "SELECT * FROM posts WHERE parent_id IS NULL")
            .fetch_all(&mut **db)
            .await
            .map_err(|err| err.into()),
    }
}
