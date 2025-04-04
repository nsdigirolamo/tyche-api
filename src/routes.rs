use uuid::Uuid;
use warp::Filter;

use crate::handlers::post;
use crate::repository::Repository;

pub fn routes(
    db: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_post(db.clone()).or(read_post(db))
}

fn create_post(
    db: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create" / "post")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(post::create_post)
}

fn read_post(
    db: Repository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("posts" / Uuid))
        .and(warp::get())
        .and_then(post::read_post)
}
