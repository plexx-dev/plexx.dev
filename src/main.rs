use rocket::*;
use rocket::fs::FileServer;
use rocket::http::Status;

use rocket_dyn_templates::{Template, context};
mod error;
mod conway;
mod games;


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
        .mount("/games/", routes![
            games::index,
        ])
        .mount("/games/conway", routes![
            conway::routes::index,
        ])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}