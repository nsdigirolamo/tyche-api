use crate::models::entities::Post;

#[derive(Debug, serde::Deserialize)]
pub struct PostInput {
    pub parent_id: Option<uuid::Uuid>,
    pub author_id: uuid::Uuid,
    pub body: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PostOutput {
    pub id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub author_id: uuid::Uuid,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Post> for PostOutput {
    fn from(post: Post) -> Self {
        PostOutput {
            id: post.id,
            parent_id: post.parent_id,
            author_id: post.author_id,
            body: post.body,
            created_at: post.created_at,
        }
    }
}

impl From<&Post> for PostOutput {
    fn from(post: &Post) -> Self {
        PostOutput {
            id: post.id,
            parent_id: post.parent_id,
            author_id: post.author_id,
            body: post.body.clone(),
            created_at: post.created_at,
        }
    }
}
