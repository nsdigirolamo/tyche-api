use std::{convert::Infallible, error::Error};

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
    } else if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed.".to_string();
    } else if rejection
        .find::<warp::reject::UnsupportedMediaType>()
        .is_some()
    {
        code = warp::http::StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = "Unsupported media type.".to_string();
    } else if let Some(err) = rejection.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = match err.source() {
            Some(cause) => format!("Bad request: {}", cause),
            None => "Bad request.".to_string(),
        };
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
