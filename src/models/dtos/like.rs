#[derive(Debug, serde::Serialize)]
pub struct LikeOutput {
    pub exists: bool,
}

impl From<bool> for LikeOutput {
    fn from(exists: bool) -> Self {
        LikeOutput { exists }
    }
}
