#[derive(serde::Serialize)]
pub struct ErrorOutput {
    pub code: u16,
    pub message: String,
}
