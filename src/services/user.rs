use regex::Regex;
use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};
use secrecy::{ExposeSecret, SecretBox};
use sha3::Digest;
use uuid::Uuid;

use crate::{
    models::{
        claims::{create_claims, encode_claims},
        dtos::{login::LoginInput, user::UserOutput},
        errors::{ErrorResponse, RepositoryError},
        login::LoginOutput,
    },
    repositories::{self, session::SessionRepository, user::UserRepository},
};

pub struct UserService {
    pub user_repository: rocket_db_pools::Connection<UserRepository>,
    pub session_repository: rocket_db_pools::Connection<SessionRepository>,
}

impl UserService {
    pub async fn create_one(self, input: LoginInput) -> Result<LoginOutput, ErrorResponse> {
        let user_password_hash = SecretBox::new(Box::new(
            sha3::Sha3_256::digest(input.password.expose_secret()).to_vec(),
        ));

        let username = input.name;
        let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        if !username_regex.is_match(&username) {
            return Err(RepositoryError::Unspecified("Invalid username".to_string()).into());
        }

        let user = match repositories::user::create_one(
            self.user_repository,
            username,
            user_password_hash,
        )
        .await
        {
            Ok(user) => user,
            Err(err) => return Err(err.into()),
        };

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

    pub async fn find_one_by_id(self, user_id: Uuid) -> Result<UserOutput, ErrorResponse> {
        match repositories::user::find_one_by_id(self.user_repository, user_id).await {
            Ok(user) => Ok(UserOutput::from(user)),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn find_one_by_name(self, user_name: String) -> Result<UserOutput, ErrorResponse> {
        match repositories::user::find_one_by_name(self.user_repository, user_name).await {
            Ok(user) => Ok(UserOutput::from(user)),
            Err(err) => Err(err.into()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserService {
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

        Outcome::Success(UserService {
            user_repository,
            session_repository,
        })
    }
}
