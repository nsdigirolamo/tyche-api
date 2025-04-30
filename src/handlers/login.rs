use rocket::{http::Status, serde::json::Json};

use crate::{
    models::{
        dtos::login::LoginInput,
        errors::ErrorResponse,
        login::{LoginData, LoginOutput},
    },
    services::login::LoginService,
};

#[rocket::post("/login", data = "<json_input>")]
pub async fn login(
    login_service: LoginService,
    json_input: Json<LoginInput>,
) -> Result<Json<LoginOutput>, ErrorResponse> {
    let input = json_input.into_inner();

    match login_service.login(input).await {
        Ok(output) => Ok(Json(output)),
        Err(err) => Err(err),
    }
}

#[rocket::post("/logout")]
pub async fn logout(
    login_service: LoginService,
    login_data: LoginData,
) -> Result<Status, ErrorResponse> {
    match login_service.logout(login_data).await {
        Some(err) => Err(err),
        None => Ok(Status::Ok),
    }
}
