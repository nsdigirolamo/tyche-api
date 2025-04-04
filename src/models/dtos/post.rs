use crate::models::entities::Post;

#[derive(serde::Deserialize)]
pub struct PostInput {
    pub author_id: uuid::Uuid,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct PostOutput {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Post> for PostOutput {
    fn from(post: Post) -> Self {
        PostOutput {
            id: post.id,
            author_id: post.author_id,
            message: post.message,
            created_at: post.created_at,
        }
    }
}
