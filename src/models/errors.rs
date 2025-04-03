use warp::reject::Reject;

#[derive(Debug)]
pub struct RepositoryError;

impl Reject for RepositoryError {}
