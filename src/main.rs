use rocket::fairing::AdHoc;
use rocket::*;
use rocket::fs::FileServer;
use rocket::http::Status;

use rocket_dyn_templates::{Template, context};

use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

mod urlshrtner;
mod error;

#[derive(Database)]
#[database("urls")]
pub struct Urls(sqlx::MySqlPool);

#[derive(Database)]
#[database("views")]
pub struct Views(sqlx::MySqlPool);

#[derive(Database)]
#[database("total_views")]
pub struct TotalViews(sqlx::MySqlPool);

/*
async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Urls::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./src/db/migrations").run(&**db).await {
          Ok(_) => Ok(rocket),
          Err(e) => {
            error!("Failed to run database migrations: {}", e);
            Err(rocket)
          }
        },
        None => Err(rocket),
    }
}
 */

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
    // let migrations_fairing = AdHoc::try_on_ignite("SQLx Migrations", run_migrations);

    rocket::build()
        .attach(Urls::init())
        .attach(Views::init())
        .attach(TotalViews::init())
        // .attach(migrations_fairing)

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
}