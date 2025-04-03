use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::Post;

#[derive(Deserialize)]
pub struct CreatePostInput {
    pub author_id: Uuid,
    pub message: String,
}

#[derive(Serialize)]
pub struct CreatePostOutput {
    pub id: Uuid,
    pub author_id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
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

#[derive(Serialize)]
pub struct ReadPostOutput {
    pub id: Uuid,
    pub author_id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
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
