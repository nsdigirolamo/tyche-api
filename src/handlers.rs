use std::convert::Infallible;

use uuid::Uuid;
use warp::http::StatusCode;

use crate::models::Post;

pub async fn create_post(post: Post) -> Result<impl warp::Reply, Infallible> {
    println!("Post Created: {:?}", post);

    Ok(StatusCode::CREATED)
}

pub async fn read_post(id: Uuid) -> Result<impl warp::Reply, Infallible> {
    let post = Post {
        id,
        title: String::from("Hello, warp"),
        body: String::from("this is a post about warp"),
    };

    Ok(warp::reply::json(&post))
}
