#[derive(rocket::Responder, Debug, serde::Serialize)]
pub enum RepositoryError {
    #[response(status = 404)]
    ResourceNotFound(&'static str),
    #[response(status = 400)]
    DuplicateResource(&'static str),
    #[response(status = 500)]
    Unspecified(&'static str),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(other: sqlx::Error) -> RepositoryError {
        match other {
            sqlx::Error::RowNotFound => {
                RepositoryError::ResourceNotFound("The requested resource could not be found.")
            }
            sqlx::Error::Database(err) => match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => {
                    RepositoryError::DuplicateResource("The requested resource already exists.")
                }
                err => {
                    println!("sqlx database error: {:?}", err);
                    RepositoryError::Unspecified("A database error occurred.")
                }
            },
            err => {
                println!("sqlx error: {:?}", err);
                RepositoryError::Unspecified("A database error occurred.")
            }
        }
    }
}
