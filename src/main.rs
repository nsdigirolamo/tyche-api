pub mod handlers;
pub mod models;
pub mod repositories;

use actix_web::{
    App, HttpServer,
    web::{Data, get, post, scope},
};
use repositories::{post::PostRepository, user::UserRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await
        .unwrap();

    let post_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await
        .unwrap();

    let user_repository = UserRepository::new(user_pool);
    let post_repository = PostRepository::new(post_pool);

    HttpServer::new(move || {
        App::new()
            .service(
                scope("/api")
                    .service(scope("health").route("", get().to(handlers::health::check)))
                    .service(
                        scope("/user")
                            .route("", post().to(handlers::user::create_one))
                            .route("/id/{user_id}", get().to(handlers::user::find_one_by_id))
                            .route(
                                "/name/{user_name}",
                                get().to(handlers::user::find_one_by_name),
                            ),
                    )
                    .service(
                        scope("/post")
                            .route("", post().to(handlers::post::create_one))
                            .route("/id/{user_id}", get().to(handlers::post::find_one_by_id))
                            .route(
                                "/id/{user_id}/children",
                                get().to(handlers::post::find_many_by_parent_id),
                            )
                            .service(
                                scope("/posts")
                                    .route("", get().to(handlers::post::find_many_without_parents)),
                            ),
                    ),
            )
            .app_data(Data::new(user_repository.clone()))
            .app_data(Data::new(post_repository.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
