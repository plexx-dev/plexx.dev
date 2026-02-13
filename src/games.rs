use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index() -> Template {
    Template::render("games", context! {})
}