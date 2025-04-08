use crate::{
    models::dtos::post::{PostInput, PostOutput},
    repositories::post::PostRepository,
};

pub async fn create_one(
    db: PostRepository,
    input: PostInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.create_one(input).await?;
    let json = warp::reply::json(&PostOutput::from(post));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn find_one_by_id(
    db: PostRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let post = db.find_one_by_id(id).await?;
    let json = warp::reply::json(&PostOutput::from(post));

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

pub async fn find_many_without_parents(
    db: PostRepository,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let posts = db
        .find_many_by_parent_id(None)
        .await?
        .iter()
        .map(PostOutput::from)
        .collect::<Vec<PostOutput>>();
    let json = warp::reply::json(&posts);

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}

pub async fn find_many_by_parent_id(
    db: PostRepository,
    parent_id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let posts = db
        .find_many_by_parent_id(Some(parent_id))
        .await?
        .iter()
        .map(PostOutput::from)
        .collect::<Vec<PostOutput>>();
    let json = warp::reply::json(&posts);

    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}
