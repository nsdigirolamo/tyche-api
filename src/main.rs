pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::routes();
    println!("Server running at http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
