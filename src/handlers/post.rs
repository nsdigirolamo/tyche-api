use rocket::{response::status, serde::json::Json};

use crate::{
    models::{
        dtos::post::{PostInput, PostOutput},
        errors::RepositoryError,
    },
    repositories::{self, post::PostRepository},
};

#[rocket::post("/", data = "<input>")]
pub async fn create_one(
    db: rocket_db_pools::Connection<PostRepository>,
    input: Json<PostInput>,
) -> Result<status::Created<Json<PostOutput>>, RepositoryError> {
    match repositories::post::create_one(db, input.into_inner()).await {
        Ok(post) => {
            let location = format!("/posts/{}", post.id);
            let output = PostOutput::from(post);
            let response = status::Created::new(location).body(Json(output));

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::get("/<post_id>", rank = 1)]
pub async fn find_one_by_id(
    db: rocket_db_pools::Connection<PostRepository>,
    post_id: uuid::Uuid,
) -> Result<Json<PostOutput>, RepositoryError> {
    match repositories::post::find_one_by_id(db, post_id).await {
        Ok(post) => {
            let output = PostOutput::from(post);
            let response = Json(output);

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::get("/")]
pub async fn find_many_without_parents(
    db: rocket_db_pools::Connection<PostRepository>,
) -> Result<Json<Vec<PostOutput>>, RepositoryError> {
    match repositories::post::find_many_by_parent_id(db, None).await {
        Ok(posts) => {
            let output: Vec<PostOutput> = posts.iter().map(PostOutput::from).collect();
            let response = Json(output);

            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[rocket::get("/<parent_id>/children")]
pub async fn find_many_by_parent_id(
    db: rocket_db_pools::Connection<PostRepository>,
    parent_id: uuid::Uuid,
) -> Result<Json<Vec<PostOutput>>, RepositoryError> {
    match repositories::post::find_many_by_parent_id(db, Some(parent_id)).await {
        Ok(posts) => {
            let output: Vec<PostOutput> = posts.iter().map(PostOutput::from).collect();
            let response = Json(output);

            Ok(response)
        }
        Err(err) => Err(err),
    }
}
