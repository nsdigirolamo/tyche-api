use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};
use secrecy::{ExposeSecret, SecretBox};
use sha3::Digest;

use crate::{
    models::{
        claims::{create_claims, decode_claims, encode_claims},
        dtos::{login::LoginInput, user::UserOutput},
        errors::{AuthError, ErrorResponse, RepositoryError},
        login::{LoginData, LoginOutput},
    },
    repositories::{self, session::SessionRepository, user::UserRepository},
};

pub struct LoginService {
    pub user_repository: rocket_db_pools::Connection<UserRepository>,
    pub session_repository: rocket_db_pools::Connection<SessionRepository>,
}

impl LoginService {
    pub async fn login(self, input: LoginInput) -> Result<LoginOutput, ErrorResponse> {
        let input_password_hash = SecretBox::new(Box::new(
            sha3::Sha3_256::digest(input.password.expose_secret()).to_vec(),
        ));

        let user =
            match repositories::user::find_one_by_name(self.user_repository, input.name).await {
                Ok(user) => user,
                Err(_) => {
                    let err = AuthError::InvalidLogin("the provided login was invalid".to_string());
                    return Err(err.into());
                }
            };

        if input_password_hash.expose_secret() != user.password_hash.expose_secret() {
            let err = AuthError::InvalidLogin("the provided login was invalid".to_string());
            return Err(err.into());
        }

        let claims = match create_claims(user.id) {
            Ok(claims) => claims,
            Err(err) => return Err(err.into()),
        };

        match repositories::session::create_one(self.session_repository, user.id, claims.jti).await
        {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        };

        let token = match encode_claims(claims) {
            Ok(token) => token,
            Err(err) => return Err(err.into()),
        };

        let login_output = LoginOutput {
            user: UserOutput::from(user),
            token,
        };

        Ok(login_output)
    }

    pub async fn logout(self, login_data: LoginData) -> Option<ErrorResponse> {
        let user_id = login_data.user.id;
        let token_id = login_data.claims.jti;

        repositories::session::delete_one(self.session_repository, user_id, token_id)
            .await
            .map(|err| err.into())
    }

    pub async fn get_login_data(self, token: String) -> Result<LoginData, ErrorResponse> {
        let claims = match decode_claims(token.to_string()) {
            Ok(claims) => claims,
            Err(err) => return Err(err.into()),
        };

        let user_id = claims.sub;

        let user = match repositories::user::find_one_by_id(self.user_repository, user_id).await {
            Ok(user) => user,
            Err(_) => return Err(AuthError::InvalidToken("invalid token".to_string()).into()),
        };

        let session_exists =
            match repositories::session::find_one(self.session_repository, claims.sub, claims.jti)
                .await
            {
                Ok(exists) => exists,
                Err(err) => return Err(err.into()),
            };

        if session_exists {
            Ok(LoginData { user, claims })
        } else {
            Err(AuthError::InvalidSession("invalid session".to_string()).into())
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginService {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_repository = match req
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

        let session_repository = match req
            .guard::<rocket_db_pools::Connection<SessionRepository>>()
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

        Outcome::Success(LoginService {
            user_repository,
            session_repository,
        })
    }
}
