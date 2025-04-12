use actix_web::{
    Either, HttpResponse,
    web::{Data, Json, Path},
};

use crate::{
    models::{
        dtos::post::{PostInput, PostOutput},
        errors::RepositoryError,
    },
    repositories::post::PostRepository,
};

type PostResult = Either<PostOutput, Result<&'static str, RepositoryError>>;
type ManyPostResult = Either<HttpResponse, Result<&'static str, RepositoryError>>;

pub async fn create_one(
    post_repository: Data<PostRepository>,
    json: Json<PostInput>,
) -> PostResult {
    let post_input = json.into_inner();
    match post_repository.create_one(post_input).await {
        Ok(post) => Either::Left(PostOutput::from(post)),
        Err(err) => Either::Right(Err(err)),
    }
}

pub async fn find_one_by_id(
    post_repository: Data<PostRepository>,
    path: Path<uuid::Uuid>,
) -> PostResult {
    let post_id = path.into_inner();
    match post_repository.find_one_by_id(post_id).await {
        Ok(post) => Either::Left(PostOutput::from(post)),
        Err(err) => Either::Right(Err(err)),
    }
}

pub async fn find_many_without_parents(post_repository: Data<PostRepository>) -> ManyPostResult {
    match post_repository.find_many_by_parent_id(None).await {
        Ok(posts) => {
            let post_outputs: Vec<PostOutput> = posts.iter().map(PostOutput::from).collect();
            Either::Left(HttpResponse::Ok().json(post_outputs))
        }
        Err(err) => Either::Right(Err(err)),
    }
}

pub async fn find_many_by_parent_id(
    post_repository: Data<PostRepository>,
    path: Path<uuid::Uuid>,
) -> ManyPostResult {
    let parent_id = Some(path.into_inner());
    match post_repository.find_many_by_parent_id(parent_id).await {
        Ok(posts) => {
            let post_outputs: Vec<PostOutput> = posts.iter().map(PostOutput::from).collect();
            Either::Left(HttpResponse::Ok().json(post_outputs))
        }
        Err(err) => Either::Right(Err(err)),
    }
}
