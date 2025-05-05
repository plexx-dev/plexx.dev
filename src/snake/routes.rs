use rocket::get;

#[get("/")]
pub async fn index() -> String {
    "gaming".to_string()
}