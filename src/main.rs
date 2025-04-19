use std::env;

use middleware::cors::CorsMiddleware;
use repositories::{post::PostRepository, user::UserRepository};
use rocket::routes;
use rocket_db_pools::Database;

pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;

#[rocket::options("/<_path..>")]
pub async fn option(_path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[rocket::launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge((
            "databases.user_repository.url",
            env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        ))
        .merge((
            "databases.post_repository.url",
            env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        ));

    rocket::custom(figment)
        .attach(CorsMiddleware)
        .attach(UserRepository::init())
        .attach(PostRepository::init())
        .mount("/api/health", routes![handlers::health::check])
        .mount(
            "/api/user",
            routes![handlers::user::create_one, handlers::user::login, option],
        )
        .mount(
            "/api/users",
            routes![
                handlers::user::find_one_by_id,
                handlers::user::find_one_by_name
            ],
        )
        .mount("/api/post", routes![handlers::post::create_one, option])
        .mount(
            "/api/posts",
            routes![
                handlers::post::find_many_without_parents,
                handlers::post::find_one_by_id,
                handlers::post::find_many_by_parent_id
            ],
        )
}
