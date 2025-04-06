pub struct UserFilter {
    pub id: Option<uuid::Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
