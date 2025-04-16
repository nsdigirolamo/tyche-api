use rocket::{response::status, serde::json::Json};
use secrecy::{ExposeSecret, SecretBox};
use sha3::Digest;

use crate::{
    models::{
        claims::encode_claims,
        dtos::user::{UserInput, UserOutput},
        errors::{AuthError, ErrorResponse, RepositoryError},
        login::LoginOutput,
    },
    repositories::{self, user::UserRepository},
};

#[rocket::post("/register", data = "<json_input>")]
pub async fn create_one(
    db: rocket_db_pools::Connection<UserRepository>,
    json_input: Json<UserInput>,
) -> Result<status::Created<Json<UserOutput>>, RepositoryError> {
    let input = json_input.into_inner();
    let user_password_hash = SecretBox::new(Box::new(
        sha3::Sha3_256::digest(input.password.expose_secret()).to_vec(),
    ));

    match repositories::user::create_one(db, input.name, user_password_hash).await {
        Ok(user) => {
            let location = format!("/users/{}", user.id);
            let output = UserOutput::from(user);
            let response = status::Created::new(location).body(Json(output));

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::get("/<user_id>", rank = 1)]
pub async fn find_one_by_id(
    db: rocket_db_pools::Connection<UserRepository>,
    user_id: uuid::Uuid,
) -> Result<Json<UserOutput>, RepositoryError> {
    match repositories::user::find_one_by_id(db, user_id).await {
        Ok(user) => {
            let output = UserOutput::from(user);
            let response = Json(output);

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::get("/<user_name>", rank = 2)]
pub async fn find_one_by_name(
    db: rocket_db_pools::Connection<UserRepository>,
    user_name: String,
) -> Result<Json<UserOutput>, RepositoryError> {
    match repositories::user::find_one_by_name(db, user_name).await {
        Ok(user) => {
            let output = UserOutput::from(user);
            let response = Json(output);

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::post("/login", data = "<json_input>")]
pub async fn login(
    db: rocket_db_pools::Connection<UserRepository>,
    json_input: Json<UserInput>,
) -> Result<Json<LoginOutput>, ErrorResponse> {
    let input = json_input.into_inner();
    let input_password_hash = SecretBox::new(Box::new(
        sha3::Sha3_256::digest(input.password.expose_secret()).to_vec(),
    ));

    let user = match repositories::user::find_one_by_name(db, input.name).await {
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

    let token = match encode_claims(user.id) {
        Ok(token) => token,
        Err(err) => return Err(err.into()),
    };

    let login_output = LoginOutput {
        user: UserOutput::from(user),
        token,
    };

    let response = Json(login_output);

    Ok(response)
}
