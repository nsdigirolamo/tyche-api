use actix_web::Responder;

pub async fn check() -> impl Responder {
    "Health is OK :)"
}
