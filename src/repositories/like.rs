use crate::models::{entities::Like, errors::RepositoryError};

#[derive(rocket_db_pools::Database)]
#[database("like_repository")]
pub struct LikeRepository(rocket_db_pools::sqlx::PgPool);

pub async fn create_one(
    mut db: rocket_db_pools::Connection<LikeRepository>,
    user_id: uuid::Uuid,
    post_id: uuid::Uuid,
) -> Option<RepositoryError> {
    let result = sqlx::query_as!(
        Like,
        "INSERT INTO likes (user_id, post_id) VALUES ($1, $2) RETURNING *",
        user_id,
        post_id,
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(_) => None,
        Err(err) => {
            println!("{:?}", err);
            Some(err.into())
        }
    }
}

pub async fn find_one(
    mut db: rocket_db_pools::Connection<LikeRepository>,
    user_id: uuid::Uuid,
    post_id: uuid::Uuid,
) -> Result<bool, RepositoryError> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM likes WHERE user_id = $1 AND post_id = $2)",
        user_id,
        post_id,
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(exists) => match exists {
            Some(value) => Ok(value),
            None => Err(RepositoryError::Unspecified(
                "something went wrong finding the like".to_string(),
            )),
        },
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_one(
    mut db: rocket_db_pools::Connection<LikeRepository>,
    user_id: uuid::Uuid,
    post_id: uuid::Uuid,
) -> Option<RepositoryError> {
    let result = sqlx::query_as!(
        Like,
        "DELETE FROM likes WHERE user_id = $1 AND post_id = $2 RETURNING *",
        user_id,
        post_id,
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(_) => None,
        Err(err) => Some(err.into()),
    }
}
