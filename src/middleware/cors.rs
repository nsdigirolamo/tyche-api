use std::env;

use once_cell::sync::Lazy;
use rocket::{
    Request,
    fairing::{Fairing, Info, Kind},
    http::Header,
};

pub struct CorsMiddleware;

static ALLOWED_ORIGIN: Lazy<String> =
    Lazy::new(|| env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set"));

#[rocket::async_trait]
impl Fairing for CorsMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            ALLOWED_ORIGIN.to_string(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}
