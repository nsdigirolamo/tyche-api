use warp::Filter;

use crate::{handlers::auth, services::auth::AuthenticationService};

pub fn routes(
    auth_service: AuthenticationService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login(auth_service)
}

pub fn login(
    auth_service: AuthenticationService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || auth_service.clone())
        .and(warp::path!("login"))
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(auth::handle_authentication)
}
