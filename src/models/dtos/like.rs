#[derive(Debug, serde::Deserialize)]
pub struct LikeInput {
    pub post_id: uuid::Uuid,
}
