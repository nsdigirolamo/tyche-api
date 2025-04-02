use sqlx::postgres::PgPoolOptions;

pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("Should be 150: {}", row.0);

    Ok(())

    // let routes = routes::routes();
    // println!("Server running at http://localhost:8080");
    // warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
