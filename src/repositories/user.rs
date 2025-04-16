use secrecy::{ExposeSecret, SecretBox};

use crate::models::entities::User;
use crate::models::errors::RepositoryError;

#[derive(rocket_db_pools::Database)]
#[database("user_repository")]
pub struct UserRepository(rocket_db_pools::sqlx::PgPool);

pub async fn create_one(
    mut db: rocket_db_pools::Connection<UserRepository>,
    user_name: String,
    user_password: SecretBox<Vec<u8>>,
) -> Result<User, RepositoryError> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, password_hash) VALUES ($1, $2) RETURNING *",
        user_name,
        user_password.expose_secret(),
    )
    .fetch_one(&mut **db)
    .await
    .map_err(|err| err.into())
}

pub async fn find_one_by_id(
    mut db: rocket_db_pools::Connection<UserRepository>,
    id: uuid::Uuid,
) -> Result<User, RepositoryError> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| err.into())
}

pub async fn find_one_by_name(
    mut db: rocket_db_pools::Connection<UserRepository>,
    name: String,
) -> Result<User, RepositoryError> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE name = $1", name)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| err.into())
}
