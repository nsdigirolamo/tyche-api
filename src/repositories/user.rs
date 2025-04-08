use crate::models::{dtos::user::UserInput, entities::User, errors::RepositoryError};

#[derive(Clone)]
pub struct UserRepository {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl UserRepository {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> UserRepository {
        UserRepository { pool }
    }

    pub async fn create_one(&self, input: UserInput) -> Result<User, RepositoryError> {
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

    pub async fn find_one_by_id(&self, id: uuid::Uuid) -> Result<User, RepositoryError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into())
    }

    pub async fn find_one_by_name(&self, name: String) -> Result<User, RepositoryError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE name = $1", name)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into())
    }
}
