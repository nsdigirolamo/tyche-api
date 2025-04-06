use handlers::rejection::handle_rejection;
use repositories::{post::PostRepository, user::UserRepository};
use warp::Filter;

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let post_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;
    let post_repository = PostRepository::new(post_pool);

    let user_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;
    let user_repository = UserRepository::new(user_pool);

    let cors = warp::cors()
        .allow_origins(vec!["http://localhost:5173"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "Access-Control-Request-Headers",
            "content-type",
        ])
        .build();
    let routes = routes::routes(user_repository, post_repository)
        .recover(handle_rejection)
        .with(cors);

    println!("Server running at http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
