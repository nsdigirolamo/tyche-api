#[derive(serde::Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}
