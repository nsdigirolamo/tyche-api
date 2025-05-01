use rocket::{response::status, serde::json::Json};

use crate::{
    models::{
        dtos::{login::LoginInput, user::UserOutput},
        errors::ErrorResponse,
        login::LoginOutput,
    },
    services::user::UserService,
};

#[rocket::post("/register", data = "<json_input>")]
pub async fn create_one(
    user_service: UserService,
    json_input: Json<LoginInput>,
) -> Result<status::Created<Json<LoginOutput>>, ErrorResponse> {
    let input = json_input.into_inner();
    let login_output = match user_service.create_one(input).await {
        Ok(output) => output,
        Err(err) => return Err(err),
    };

    let location = format!("/users/{}", login_output.user.id);
    let response = status::Created::new(location).body(Json(login_output));

    Ok(response)
}

#[rocket::get("/<user_id>", rank = 1)]
pub async fn find_one_by_id(
    user_service: UserService,
    user_id: uuid::Uuid,
) -> Result<Json<UserOutput>, ErrorResponse> {
    let user_output = match user_service.find_one_by_id(user_id).await {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    let response = Json(user_output);

    Ok(response)
}

#[rocket::get("/<user_name>", rank = 2)]
pub async fn find_one_by_name(
    user_service: UserService,
    user_name: String,
) -> Result<Json<UserOutput>, ErrorResponse> {
    let user_output = match user_service.find_one_by_name(user_name).await {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    let response = Json(user_output);

    Ok(response)
}
