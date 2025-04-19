use secrecy::SecretBox;

#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub password_hash: SecretBox<[u8]>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Post {
    pub id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub author_id: uuid::Uuid,
    pub body: String,
    pub like_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Like {
    pub user_id: uuid::Uuid,
    pub post_id: uuid::Uuid,
}
