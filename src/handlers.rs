use uuid::Uuid;
use warp::reject::Rejection;

use crate::{
    models::dtos::{CreatePostInput, CreatePostOutput, ReadPostOutput},
    repository::Repository,
};

pub async fn create_post(
    db: Repository,
    input: CreatePostInput,
) -> Result<impl warp::Reply, Rejection> {
    let post = db.create_post(input).await?;
    Ok(warp::reply::json(&CreatePostOutput::from(post)))
}

pub async fn read_post(db: Repository, id: Uuid) -> Result<impl warp::Reply, Rejection> {
    let post = db.get_post(id).await?;
    Ok(warp::reply::json(&ReadPostOutput::from(post)))
}
