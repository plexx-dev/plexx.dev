use rocket::get;

#[get("/")]
pub async fn index() -> String {
    "test".to_string()
}