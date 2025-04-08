use uuid::Uuid;
use warp::Filter;

use crate::{handlers::post, repositories::post::PostRepository};

pub fn routes(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_one(db.clone())
        .or(find_one_by_id(db.clone()))
        .or(find_many_without_parents(db.clone()))
        .or(find_many_by_parent_id(db.clone()))
}

fn create_one(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("post")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(post::create_one)
}

fn find_one_by_id(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("post" / Uuid))
        .and(warp::get())
        .and_then(post::find_one_by_id)
}

fn find_many_without_parents(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("posts"))
        .and(warp::get())
        .and_then(post::find_many_without_parents)
}

fn find_many_by_parent_id(
    db: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("post" / Uuid / "children"))
        .and(warp::get())
        .and_then(post::find_many_by_parent_id)
}
