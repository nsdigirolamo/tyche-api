// use std::fmt;

// use actix_web::{
//     HttpResponse,
//     http::{StatusCode, header::ContentType},
// };

use std::fmt;

use actix_web::{
    HttpResponse,
    http::{StatusCode, header::ContentType},
};

#[derive(Debug, serde::Serialize)]
pub enum RepositoryError {
    ResourceNotFound,
    DuplicateResource,
    Unspecified,
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match *self {
            RepositoryError::ResourceNotFound => "The requested resource could not be found.",
            RepositoryError::DuplicateResource => "The requested resource already exists.",
            RepositoryError::Unspecified => "A database error occurred.",
        };
        write!(f, "{}", message)
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(other: sqlx::Error) -> RepositoryError {
        match other {
            sqlx::Error::RowNotFound => RepositoryError::ResourceNotFound,
            sqlx::Error::Database(err) => match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => RepositoryError::DuplicateResource,
                err => {
                    println!("sqlx database error: {:?}", err);
                    RepositoryError::Unspecified
                }
            },
            err => {
                println!("sqlx error: {:?}", err);
                RepositoryError::Unspecified
            }
        }
    }
}

impl actix_web::error::ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RepositoryError::ResourceNotFound => StatusCode::NOT_FOUND,
            RepositoryError::DuplicateResource => StatusCode::BAD_REQUEST,
            RepositoryError::Unspecified => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
