#[rocket::get("/check")]
pub fn check() -> &'static str {
    "Health is OK :)"
}
