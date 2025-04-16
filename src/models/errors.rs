#[derive(rocket::Responder, Debug, serde::Serialize)]
pub enum ErrorResponse {
    Repository(RepositoryError),
    Auth(AuthError),
}

impl From<RepositoryError> for ErrorResponse {
    fn from(err: RepositoryError) -> Self {
        ErrorResponse::Repository(err)
    }
}

impl From<AuthError> for ErrorResponse {
    fn from(err: AuthError) -> Self {
        ErrorResponse::Auth(err)
    }
}

#[derive(rocket::Responder, Debug, serde::Serialize)]
pub enum RepositoryError {
    #[response(status = 404)]
    ResourceNotFound(String),
    #[response(status = 400)]
    DuplicateResource(String),
    #[response(status = 500)]
    Unspecified(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(other: sqlx::Error) -> RepositoryError {
        match other {
            sqlx::Error::RowNotFound => RepositoryError::ResourceNotFound(
                "the requested resource could not be found".to_string(),
            ),
            sqlx::Error::Database(err) => match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => RepositoryError::DuplicateResource(
                    "the requested resource already exists".to_string(),
                ),
                err => {
                    println!("sqlx database error: {:?}", err);
                    RepositoryError::Unspecified("a database error occurred".to_string())
                }
            },
            err => {
                println!("sqlx error: {:?}", err);
                RepositoryError::Unspecified("a database error occurred".to_string())
            }
        }
    }
}

#[derive(rocket::Responder, Debug, serde::Serialize)]
pub enum AuthError {
    #[response(status = 400)]
    InvalidToken(String),
    #[response(status = 400)]
    InvalidLogin(String),
    #[response(status = 500)]
    Unspecified(String),
}
