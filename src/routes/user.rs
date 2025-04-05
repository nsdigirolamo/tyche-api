use uuid::Uuid;
use warp::Filter;

use crate::{handlers::user, repositories::user::UserRepository};

pub fn routes(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_user(db.clone())
        .or(read_user(db.clone()))
        .or(update_user(db.clone()))
        .or(delete_user(db.clone()))
}

fn create_user(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(warp::any().map(move || db.clone()))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(user::create_user)
}

fn read_user(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("user" / Uuid))
        .and(warp::get())
        .and_then(user::read_user)
}

fn update_user(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("user" / Uuid))
        .and(warp::put())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(user::update_user)
}

fn delete_user(
    db: UserRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || db.clone())
        .and(warp::path!("user" / Uuid))
        .and(warp::delete())
        .and_then(user::delete_user)
}
