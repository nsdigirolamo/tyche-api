use crate::models::entities::Post;

#[derive(serde::Deserialize)]
pub struct PostInput {
    pub parent_id: Option<uuid::Uuid>,
    pub author_name: String,
    pub body: String,
}

#[derive(serde::Serialize)]
pub struct PostOutput {
    pub id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub author_name: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Post> for PostOutput {
    fn from(post: Post) -> Self {
        PostOutput {
            id: post.id,
            parent_id: post.parent_id,
            author_name: post.author_name,
            body: post.body,
            created_at: post.created_at,
        }
    }
}
