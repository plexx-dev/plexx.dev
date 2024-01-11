use rocket::*;
use rocket::fs::FileServer;
use rocket::http::Status;

use rocket_dyn_templates::{Template, context};

use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

mod urlshrtner;
mod error;

#[derive(Database)]
#[database("site")]
pub struct Urls(sqlx::SqlitePool);

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[catch(default)]
async fn default_catcher(status: Status, _: &Request<'_>) -> Template {
    Template::render("error", context! {
        code: status.code,
        message: error::get_error_message(status.code)
    })
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index
        ])
        .mount("/", routes![
            urlshrtner::routes::get_url,
            urlshrtner::routes::index,
            urlshrtner::routes::submit,           
        ])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
        .attach(Urls::init())
}