use secrecy::SecretBox;

#[derive(Debug, serde::Deserialize)]
pub struct LoginInput {
    pub name: String,
    pub password: SecretBox<String>,
}
