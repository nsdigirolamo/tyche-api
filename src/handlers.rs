use std::convert::Infallible;

use chrono::Utc;
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
        author_id: Uuid::new_v4(),
        message: String::from("hello world"),
        created_at: Utc::now(),
    };

    Ok(warp::reply::json(&post))
}
