#[derive(Debug)]
pub struct Post {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
