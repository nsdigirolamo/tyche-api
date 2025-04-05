use uuid::Uuid;
use warp::Filter;

use crate::{handlers::post, repositories::post::PostRepository};

pub fn routes(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_post(db.clone())
        .or(read_post(db.clone()))
        .or(update_post(db.clone()))
        .or(delete_post(db.clone()))
}

fn create_post(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("post")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(post::create)
}

fn read_post(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("post" / Uuid))
        .and(warp::get())
        .and_then(post::read)
}

fn update_post(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("post" / Uuid))
        .and(warp::put())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(post::update)
}

fn delete_post(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("post" / Uuid))
        .and(warp::delete())
        .and_then(post::delete)
}
