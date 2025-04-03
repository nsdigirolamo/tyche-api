use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
