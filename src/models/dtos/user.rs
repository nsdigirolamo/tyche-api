use crate::models::entities::User;

#[derive(serde::Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct UserOutput {
    pub id: uuid::Uuid,
    pub username: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        UserOutput {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}
