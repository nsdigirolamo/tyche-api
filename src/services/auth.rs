use crate::{
    models::{credential::LoginCredential, entities::User, errors::AuthError},
    repositories::user::UserRepository,
};

#[derive(Debug, Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
}

impl AuthenticationService {
    pub fn new(user_repository: UserRepository) -> AuthenticationService {
        AuthenticationService { user_repository }
    }

    pub async fn authenticate(
        &self,
        given_username: String,
        given_password: String,
    ) -> Result<User, AuthError> {
        let user = self
            .user_repository
            .find_one_by_name(given_username)
            .await
            .map_err(|_| AuthError {
                status: warp::http::StatusCode::BAD_REQUEST,
                message: "Incorrect username and/or password.".to_string(),
            })?;

        if given_password != user.password {
            return Err(AuthError {
                status: warp::http::StatusCode::BAD_REQUEST,
                message: "Incorrect username and/or password.".to_string(),
            });
        }

        Ok(user)
    }

    pub async fn authorize(&self, cred: LoginCredential) -> Option<AuthError> {
        let user = match self.user_repository.find_one_by_id(cred.user_id).await {
            Ok(user) => user,
            Err(_) => {
                return Some(AuthError {
                    status: warp::http::StatusCode::UNAUTHORIZED,
                    message: "Invalid credential.".to_string(),
                });
            }
        };

        let is_cred_valid = user.id == cred.user_id
            && user.name == cred.user_name
            && user.password == cred.user_password;

        if !is_cred_valid {
            return Some(AuthError {
                status: warp::http::StatusCode::UNAUTHORIZED,
                message: "Invalid credential.".to_string(),
            });
        }

        None
    }
}
