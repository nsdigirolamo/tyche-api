use std::{env, fs};

use middleware::cors::CorsMiddleware;
use repositories::{
    like::LikeRepository, post::PostRepository, session::SessionRepository, user::UserRepository,
};
use rocket::routes;
use rocket_db_pools::Database;

pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;

#[rocket::options("/<_path..>")]
pub async fn option(_path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[rocket::launch]
fn rocket() -> _ {
    let environment_type = env::var("ENVIRONMENT_TYPE").expect("ENVIRONMENT_TYPE must be set");
    let host_address = match environment_type.as_str() {
        "development" => "127.0.0.1",
        "production" => "0.0.0.0",
        _ => panic!("invalid environment type"),
    };

    let database_url =
        fs::read_to_string(env::var("DATABASE_URL_FILE").expect("DATABASE_URL_FILE must be set"))
            .expect("DATABASE_URL_FILE must be readable");

    let figment = rocket::Config::figment()
        .merge(("address", host_address))
        .merge(("databases.user_repository.url", database_url.clone()))
        .merge(("databases.post_repository.url", database_url.clone()))
        .merge(("databases.like_repository.url", database_url.clone()))
        .merge(("databases.session_repository.url", database_url.clone()));

    rocket::custom(figment)
        .attach(CorsMiddleware)
        .attach(UserRepository::init())
        .attach(PostRepository::init())
        .attach(LikeRepository::init())
        .attach(SessionRepository::init())
        .mount(
            "/api",
            routes![handlers::login::login, handlers::login::logout, option],
        )
        .mount("/api/health", routes![handlers::health::check])
        .mount("/api/user", routes![handlers::user::create_one])
        .mount(
            "/api/users",
            routes![
                handlers::user::find_one_by_id,
                handlers::user::find_one_by_name
            ],
        )
        .mount("/api/post", routes![handlers::post::create_one])
        .mount(
            "/api/posts",
            routes![
                handlers::post::find_many_without_parents,
                handlers::post::find_one_by_id,
                handlers::post::find_many_by_parent_id
            ],
        )
        .mount(
            "/api/like",
            routes![handlers::like::create_one, handlers::like::delete_one],
        )
        .mount("/api/likes", routes![handlers::like::find_one])
}
