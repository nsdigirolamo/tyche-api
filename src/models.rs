use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}
