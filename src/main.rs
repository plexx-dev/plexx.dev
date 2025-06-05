use rocket::fairing::{Fairing, Info, Kind};
use rocket::shield::{NoSniff, Shield};
use rocket::{http::ContentType, *};
use rocket::fs::FileServer;
use rocket::http::Status;

use rocket_dyn_templates::{Template, context};
mod error;
mod conway;
mod games;
mod snake;

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

pub struct WasmMimeFairing;

#[rocket::async_trait]
impl Fairing for WasmMimeFairing {
    fn info(&self) -> Info {
        Info {
            name: "WASM MIME Type Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.uri().path().ends_with(".wasm") {
            response.set_header(ContentType::new("application", "wasm"));
            response.remove_header("x-content-type-options");
            response.remove_header("permissions-policy");
        }
    }
}



#[launch]
async fn rocket() -> _ {
    let shield = Shield::default()
    .disable::<NoSniff>();

    rocket::build()
        .attach(shield)
        .attach(WasmMimeFairing)
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
        .mount("/static", FileServer::from("static"))
        .mount("/game_files", FileServer::from("game_files"))
        //.mount("/", routes![serve_static]) // Custom static file handler
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}