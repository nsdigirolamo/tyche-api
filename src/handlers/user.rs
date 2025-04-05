use crate::{
    models::dtos::user::{UserInput, UserOutput},
    repositories::{Repository, user::UserRepository},
};

pub async fn create_user(
    db: UserRepository,
    input: UserInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.create(input).await?;
    let json = warp::reply::json(&UserOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn read_user(
    db: UserRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.read(id).await?;
    let json = warp::reply::json(&UserOutput::from(post));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

pub async fn update_user(
    db: UserRepository,
    id: uuid::Uuid,
    input: UserInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.update(id, input).await?;
    let json = warp::reply::json(&UserOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete_user(
    db: UserRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    match db.delete(id).await {
        Some(err) => Err(err.into()),
        None => Ok(warp::http::StatusCode::OK),
    }
}
