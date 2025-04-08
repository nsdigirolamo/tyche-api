use uuid::Uuid;
use warp::Filter;

use crate::{handlers::user, repositories::user::UserRepository};

pub fn routes(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_one(db.clone())
        .or(create_one(db.clone()))
        .or(find_one_by_id(db.clone()))
        .or(find_one_by_name(db.clone()))
}

fn create_one(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(user::create_one)
}

fn find_one_by_id(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("user" / Uuid))
        .and(warp::get())
        .and_then(user::find_one_by_id)
}

fn find_one_by_name(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("user" / String))
        .and(warp::get())
        .and_then(user::find_one_by_name)
}
