use uuid::Uuid;

use crate::models::{entities::Session, errors::RepositoryError};

#[derive(rocket_db_pools::Database)]
#[database("session_repository")]
pub struct SessionRepository(rocket_db_pools::sqlx::PgPool);

pub async fn create_one(
    mut db: rocket_db_pools::Connection<SessionRepository>,
    user_id: Uuid,
    token_id: Uuid,
) -> Result<Session, RepositoryError> {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token_id) VALUES ($1, $2) RETURNING *",
        user_id,
        token_id
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|err| err.into())
}

pub async fn find_one(
    mut db: rocket_db_pools::Connection<SessionRepository>,
    user_id: Uuid,
    token_id: Uuid,
) -> Result<bool, RepositoryError> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM sessions WHERE user_id = $1 AND token_id = $2)",
        user_id,
        token_id
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(exists) => match exists {
            Some(value) => Ok(value),
            None => Err(RepositoryError::Unspecified(
                "something went wrong finding the session".to_string(),
            )),
        },
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_one(
    mut db: rocket_db_pools::Connection<SessionRepository>,
    user_id: Uuid,
    token_id: Uuid,
) -> Option<RepositoryError> {
    let result = sqlx::query_as!(
        Session,
        "DELETE FROM sessions WHERE user_id = $1 AND token_id = $2 RETURNING *",
        user_id,
        token_id,
    )
    .fetch_one(&mut **db)
    .await;

    match result {
        Ok(_) => None,
        Err(err) => Some(err.into()),
    }
}
