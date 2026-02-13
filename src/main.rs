use rocket::{catch, catchers, get, launch, routes, Request};
use rocket::shield::Shield;
use rocket::fs::FileServer;
use rocket::http::Status;

use rocket_dyn_templates::{Template, context};
mod error;
mod conway;
mod games;
mod snake;
mod ping_pong;

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
    let shield = Shield::default();

    rocket::build()
        .attach(shield)
        .mount("/", routes![
            index
        ])
        .mount("/games/", routes![
            games::index,
        ])
        .mount("/games/snake", routes![
            snake::routes::index,
        ])
        .mount("/games/conway", routes![
            conway::routes::index,
        ])
        .mount("/games/ping_pong", routes![
            ping_pong::routes::index,
        ])
        .mount("/static", FileServer::from("static"))
        .mount("/game_files", FileServer::from("game_files"))
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}