use uuid::Uuid;
use warp::Filter;

use crate::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_post().or(read_post())
}

fn create_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create" / "post")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(handlers::create_post)
}

fn read_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / Uuid)
        .and(warp::get())
        .and_then(handlers::read_post)
}
