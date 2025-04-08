use crate::{
    models::dtos::user::{UserInput, UserOutput},
    repositories::user::UserRepository,
};

pub async fn create_one(
    db: UserRepository,
    input: UserInput,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let user = db.create_one(input).await?;
    let json = warp::reply::json(&UserOutput::from(user));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn find_one_by_id(
    db: UserRepository,
    id: uuid::Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let user = db.find_one_by_id(id).await?;
    let json = warp::reply::json(&UserOutput::from(user));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn find_one_by_name(
    db: UserRepository,
    name: String,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let user = db.find_one_by_name(name).await?;
    let json = warp::reply::json(&UserOutput::from(user));

    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::CREATED,
    ))
}
