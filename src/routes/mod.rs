use warp::Filter;

use crate::{
    repositories::{post::PostRepository, user::UserRepository},
    services::auth::AuthenticationService,
};

pub mod auth;
pub mod post;
pub mod user;

pub fn routes(
    auth_service: AuthenticationService,
    user_repository: UserRepository,
    post_repository: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post::routes(post_repository)
        .or(user::routes(user_repository))
        .or(auth::routes(auth_service))
}
