use crate::{
    models::dtos::post::{CreatePostInput, CreatePostOutput, ReadPostOutput},
    repository::Repository,
};

pub async fn create_post(
    db: Repository,
    input: CreatePostInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.create_post(input).await?;
    let json = warp::reply::json(&CreatePostOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn read_post(
    db: Repository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.get_post(id).await?;
    let json = warp::reply::json(&ReadPostOutput::from(post));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}
