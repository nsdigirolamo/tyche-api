use crate::models::entities::User;

#[derive(Debug, serde::Serialize)]
pub struct UserOutput {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        UserOutput {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
        }
    }
}
