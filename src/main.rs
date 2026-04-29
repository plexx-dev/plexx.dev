use std::path::Path;

use rocket::fs::{FileServer, NamedFile};
use rocket::http::Status;
use rocket::shield::Shield;
use rocket::{Request, catch, catchers, get, launch, routes};

use rocket_dyn_templates::{Template, context};
mod conway;
mod error;
mod games;
mod ping_pong;
mod snake;

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/robots.txt")).await.ok()
}

#[get("/teapot")]
async fn teapot() -> Template {
    Template::render(
        "error",
        context! {
            code: 418,
            message: error::get_error_message(418)
        },
    )
}

#[catch(default)]
async fn default_catcher(status: Status, _: &Request<'_>) -> Template {
    Template::render(
        "error",
        context! {
            code: status.code,
            message: error::get_error_message(status.code)
        },
    )
}

#[launch]
async fn rocket() -> _ {
    let shield = Shield::default();

    rocket::build()
        .attach(shield)
        .mount("/", routes![index, robots, teapot])
        .mount("/games/", routes![games::index,])
        .mount("/games/snake", routes![snake::routes::index,])
        .mount("/games/conway", routes![conway::routes::index,])
        .mount("/games/ping_pong", routes![ping_pong::routes::index,])
        .mount("/static", FileServer::from("static"))
        .mount("/game_files", FileServer::from("game_files"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}
