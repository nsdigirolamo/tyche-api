use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
