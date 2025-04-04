#[derive(Debug)]
pub struct Post {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
