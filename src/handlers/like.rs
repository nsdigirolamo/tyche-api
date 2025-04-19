use rocket::http::Status;

use crate::{
    models::{errors::RepositoryError, login::LoginData},
    repositories::{self, like::LikeRepository},
};

#[rocket::post("/<post_id>")]
pub async fn create_one(
    db: rocket_db_pools::Connection<LikeRepository>,
    login_data: LoginData,
    post_id: uuid::Uuid,
) -> Result<Status, RepositoryError> {
    let user_id = login_data.user.id;

    match repositories::like::create_one(db, user_id, post_id).await {
        Some(err) => Err(err),
        None => Ok(Status::Ok),
    }
}

#[rocket::delete("/<post_id>")]
pub async fn delete_one(
    db: rocket_db_pools::Connection<LikeRepository>,
    login_data: LoginData,
    post_id: uuid::Uuid,
) -> Result<Status, RepositoryError> {
    let user_id = login_data.user.id;

    match repositories::like::delete_one(db, user_id, post_id).await {
        Some(err) => Err(err),
        None => Ok(Status::Ok),
    }
}
