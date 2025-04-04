use std::convert::Infallible;

use crate::models::{dtos::error::ErrorOutput, errors::RepositoryError};

pub async fn handle_rejection(
    rejection: warp::reject::Rejection,
) -> Result<impl warp::reply::Reply, Infallible> {
    let code: warp::http::StatusCode;
    let message: String;

    if rejection.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not found.".to_string();
    } else if let Some(err) = rejection.find::<RepositoryError>() {
        code = err.status;
        message = err.message.clone();
    } else {
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Some error occured.".to_string();
    }

    let json = warp::reply::json(&ErrorOutput {
        code: code.into(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
