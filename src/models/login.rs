use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};
use uuid::Uuid;

use crate::{
    models::{
        claims::{Claims, decode_claims},
        entities::User,
        errors::{AuthError, ErrorResponse, RepositoryError},
    },
    repositories::{self, user::UserRepository},
};

use super::dtos::user::UserOutput;

#[derive(Debug, serde::Serialize)]
pub struct LoginOutput {
    pub user: UserOutput,
    pub token: String,
}

#[derive(Debug)]
pub struct LoginData {
    pub user: User,
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginData {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = match req
            .guard::<rocket_db_pools::Connection<UserRepository>>()
            .await
        {
            Outcome::Success(conn) => conn,
            Outcome::Error(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    RepositoryError::RepositoryNotFound(
                        "unable to connect to database".to_string(),
                    )
                    .into(),
                ));
            }
            Outcome::Forward(forward) => return Outcome::Forward(forward),
        };

        let token = match req.headers().get_one("authorization") {
            Some(value) => value.trim_start_matches("Bearer").trim(),
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AuthError::Unauthenticated("no token was provided".to_string()).into(),
                ));
            }
        };

        let claims = match decode_claims(token.to_string()) {
            Ok(claims) => claims,
            Err(err) => {
                return Outcome::Error((Status::Unauthorized, ErrorResponse::from(err)));
            }
        };

        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ErrorResponse::from(AuthError::InvalidToken("invalid token".to_string())),
                ));
            }
        };

        let user = match repositories::user::find_one_by_id(db, user_id).await {
            Ok(user) => user,
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ErrorResponse::from(AuthError::InvalidToken("invalid token".to_string())),
                ));
            }
        };

        Outcome::Success(LoginData { user, claims })
    }
}
