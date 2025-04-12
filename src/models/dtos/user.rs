use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};

use crate::models::entities::User;

#[derive(serde::Deserialize)]
pub struct UserInput {
    pub name: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct UserOutput {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserOutput {
    fn from(user: User) -> Self {
        UserOutput {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
        }
    }
}

impl Responder for UserOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
