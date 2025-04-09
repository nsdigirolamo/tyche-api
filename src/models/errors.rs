#[derive(Debug)]
pub struct RepositoryError {
    pub status: warp::http::StatusCode,
    pub message: String,
}

impl warp::reject::Reject for RepositoryError {}

impl From<sqlx::Error> for RepositoryError {
    fn from(other: sqlx::Error) -> RepositoryError {
        match other {
            sqlx::Error::RowNotFound => RepositoryError {
                status: warp::http::StatusCode::NOT_FOUND,
                message: "The requested resource could not be found.".to_string(),
            },
            _ => {
                println!("{}", other);
                RepositoryError {
                    status: warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: "A database error occured.".to_string(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct AuthError {
    pub status: warp::http::StatusCode,
    pub message: String,
}

impl warp::reject::Reject for AuthError {}
