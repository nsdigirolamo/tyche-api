use chrono::{DateTime, Utc};
use secrecy::SecretBox;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: SecretBox<[u8]>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Post {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub author_id: Uuid,
    pub body: String,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Like {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Debug)]
pub struct Session {
    pub user_id: Uuid,
    pub token_id: Uuid,
}
