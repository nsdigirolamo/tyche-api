use crate::{
    models::dtos::user::{UserInput, UserOutput},
    repositories::{Repository, user::UserRepository},
};

pub async fn create(
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

pub async fn read(
    db: UserRepository,
    name: String,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.read(name).await?;
    let json = warp::reply::json(&UserOutput::from(post));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

pub async fn update(
    db: UserRepository,
    name: String,
    input: UserInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.update(name, input).await?;
    let json = warp::reply::json(&UserOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete(
    db: UserRepository,
    name: String,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    match db.delete(name).await {
        Some(err) => Err(err.into()),
        None => Ok(warp::http::StatusCode::OK),
    }
}
