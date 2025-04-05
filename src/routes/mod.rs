use warp::Filter;

use crate::repositories::{post::PostRepository, user::UserRepository};

pub mod post;
pub mod user;

pub fn routes(
    user_repository: UserRepository,
    post_repository: PostRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post::routes(post_repository).or(user::routes(user_repository))
}
