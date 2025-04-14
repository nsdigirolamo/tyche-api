use rocket::{response::status, serde::json::Json};

use crate::{
    models::{
        dtos::user::{UserInput, UserOutput},
        errors::RepositoryError,
    },
    repositories::{self, user::UserRepository},
};

#[rocket::post("/", data = "<input>")]
pub async fn create_one(
    db: rocket_db_pools::Connection<UserRepository>,
    input: Json<UserInput>,
) -> Result<status::Created<Json<UserOutput>>, RepositoryError> {
    match repositories::user::create_one(db, input.into_inner()).await {
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
