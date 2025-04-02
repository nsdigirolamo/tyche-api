use models::Post;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;

    let author_id = Uuid::new_v4();
    let message = "Hello world.";

    sqlx::query!(
        "INSERT INTO posts (author_id, message) VALUES ($1, $2)",
        author_id,
        message
    )
    .execute(&pool)
    .await?;

    let row = sqlx::query_as!(
        Post,
        "SELECT id, author_id, message, created_at FROM posts WHERE author_id = $1",
        author_id,
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", row);

    Ok(())

    // let routes = routes::routes();
    // println!("Server running at http://localhost:8080");
    // warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
