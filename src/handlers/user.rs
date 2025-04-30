use regex::Regex;
use rocket::{response::status, serde::json::Json};
use secrecy::{ExposeSecret, SecretBox};
use sha3::Digest;

use crate::{
    models::{
        claims::{create_claims, encode_claims},
        dtos::{login::LoginInput, user::UserOutput},
        errors::{ErrorResponse, RepositoryError},
        login::LoginOutput,
    },
    repositories::{self, user::UserRepository},
};

#[rocket::post("/register", data = "<json_input>")]
pub async fn create_one(
    db: rocket_db_pools::Connection<UserRepository>,
    json_input: Json<LoginInput>,
) -> Result<status::Created<Json<LoginOutput>>, ErrorResponse> {
    let input = json_input.into_inner();
    let user_password_hash = SecretBox::new(Box::new(
        sha3::Sha3_256::digest(input.password.expose_secret()).to_vec(),
    ));

    let username = input.name;
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !username_regex.is_match(&username) {
        return Err(RepositoryError::Unspecified("Invalid username".to_string()).into());
    }

    let user = match repositories::user::create_one(db, username, user_password_hash).await {
        Ok(user) => user,
        Err(err) => return Err(err.into()),
    };

    let claims = match create_claims(user.id) {
        Ok(claims) => claims,
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

    let location = format!("/users/{}", login_output.user.id);
    let response = status::Created::new(location).body(Json(login_output));

    Ok(response)
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
