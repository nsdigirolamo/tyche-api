use std::convert::Infallible;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::with_status;
use warp::{
    reject::Rejection,
    reply::{Reply, json},
};

use crate::models::dtos::ErrorOutput;
use crate::models::errors::RepositoryError;
use crate::{
    models::dtos::{CreatePostInput, CreatePostOutput, ReadPostOutput},
    repository::Repository,
};

pub async fn create_post(db: Repository, input: CreatePostInput) -> Result<impl Reply, Rejection> {
    let post = db.create_post(input).await?;
    let json = json(&CreatePostOutput::from(post));

    Ok(with_status(json, StatusCode::CREATED))
}

pub async fn read_post(db: Repository, id: Uuid) -> Result<impl Reply, Rejection> {
    let post = db.get_post(id).await?;
    let json = json(&ReadPostOutput::from(post));

    Ok(with_status(json, StatusCode::OK))
}

pub async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Infallible> {
    let code: StatusCode;
    let message: String;

    if rejection.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found.".to_string();
    } else if let Some(err) = rejection.find::<RepositoryError>() {
        code = err.status;
        message = err.message.clone();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Some error occured.".to_string();
    }

    let json = warp::reply::json(&ErrorOutput {
        code: code.into(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
