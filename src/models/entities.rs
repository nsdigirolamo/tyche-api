#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Post {
    pub id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub author_id: uuid::Uuid,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
