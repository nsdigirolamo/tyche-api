use crate::{models::dtos::auth::LoginInput, services::auth::AuthenticationService};

pub async fn handle_authentication(
    auth_service: AuthenticationService,
    input: LoginInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    match auth_service
        .authenticate(input.username, input.password)
        .await
    {
        Ok(_) => Ok(warp::reply::with_header(
            "",
            "set-cookie",
            format!(
                "sessionId={}; Domain=localhost; HttpOnly; Path=/; SameSite=Strict",
                uuid::Uuid::new_v4(),
            ),
        )),
        Err(err) => Err(err.into()),
    }
}
