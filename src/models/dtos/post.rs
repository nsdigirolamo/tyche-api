use crate::models::entities::Post;

#[derive(serde::Deserialize)]
pub struct CreatePostInput {
    pub author_id: uuid::Uuid,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct CreatePostOutput {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Post> for CreatePostOutput {
    fn from(post: Post) -> Self {
        CreatePostOutput {
            id: post.id,
            author_id: post.author_id,
            message: post.message,
            created_at: post.created_at,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ReadPostOutput {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Post> for ReadPostOutput {
    fn from(post: Post) -> Self {
        ReadPostOutput {
            id: post.id,
            author_id: post.author_id,
            message: post.message,
            created_at: post.created_at,
        }
    }
}
