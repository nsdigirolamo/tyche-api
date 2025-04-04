use handlers::rejection::handle_rejection;
use repository::Repository;
use sqlx::postgres::PgPoolOptions;
use warp::Filter;

pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;
    let db = Repository::new(pool);

    let routes = routes::routes(db).recover(handle_rejection);
    println!("Server running at http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
