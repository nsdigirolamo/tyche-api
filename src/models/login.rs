use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::{
    models::{
        claims::Claims,
        entities::User,
        errors::{AuthError, ErrorResponse},
    },
    services::login::LoginService,
};

use super::{dtos::user::UserOutput, errors::ServiceError};

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
        let login_service = match req.guard::<LoginService>().await {
            Outcome::Success(service) => service,
            Outcome::Error(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ServiceError::ServiceNotFound("failed to create login service".to_string())
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

        let login_data = match login_service.get_login_data(token.to_string()).await {
            Ok(login_data) => login_data,
            Err(err) => return Outcome::Error((Status::Unauthorized, err)),
        };

        Outcome::Success(login_data)
    }
}
