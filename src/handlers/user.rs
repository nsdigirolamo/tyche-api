use actix_web::{
    Either,
    web::{self},
};

use crate::{
    models::{
        dtos::user::{UserInput, UserOutput},
        errors::RepositoryError,
    },
    repositories::user::UserRepository,
};

type UserResult = Either<UserOutput, Result<&'static str, RepositoryError>>;

pub async fn create_one(
    user_repository: web::Data<UserRepository>,
    json: web::Json<UserInput>,
) -> UserResult {
    let user_input = json.into_inner();
    match user_repository.create_one(user_input).await {
        Ok(user) => Either::Left(UserOutput::from(user)),
        Err(err) => Either::Right(Err(err)),
    }
}

pub async fn find_one_by_id(
    user_repository: web::Data<UserRepository>,
    path: web::Path<uuid::Uuid>,
) -> UserResult {
    let user_id = path.into_inner();
    match user_repository.find_one_by_id(user_id).await {
        Ok(user) => Either::Left(UserOutput::from(user)),
        Err(err) => Either::Right(Err(err)),
    }
}

pub async fn find_one_by_name(
    user_repository: web::Data<UserRepository>,
    path: web::Path<String>,
) -> UserResult {
    let user_name = path.into_inner();
    match user_repository.find_one_by_name(user_name).await {
        Ok(user) => Either::Left(UserOutput::from(user)),
        Err(err) => Either::Right(Err(err)),
    }
}
