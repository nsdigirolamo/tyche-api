use crate::{
    models::dtos::post::{PostInput, PostOutput},
    repositories::{Repository, post::PostRepository},
};

pub async fn create_post(
    db: PostRepository,
    input: PostInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.create(input).await?;
    let json = warp::reply::json(&PostOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn read_post(
    db: PostRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.read(id).await?;
    let json = warp::reply::json(&PostOutput::from(post));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

pub async fn update_post(
    db: PostRepository,
    id: uuid::Uuid,
    input: PostInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.update(id, input).await?;
    let json = warp::reply::json(&PostOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete_post(
    db: PostRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    match db.delete(id).await {
        Some(err) => Err(err.into()),
        None => Ok(warp::http::StatusCode::OK),
    }
}
