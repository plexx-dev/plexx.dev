use rocket::*;

use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[allow(unused_imports)]
use rocket_db_pools::{Connection, Database};
use rocket_db_pools::sqlx::{self, Row};

use validator::{Validate, validate_url};

use crate::Urls;

use super::short;

#[get("/<url_hash>")]
pub async fn get_url(mut db: Connection<Urls>, url_hash: String) -> Redirect {
    let fallback_url = String::from("https://plexx.dev");
    
    let url: Option<String> = match sqlx::query("SELECT url FROM urls WHERE url_hash = ?")
        .bind(&url_hash)
        .fetch_one(&mut*db)
        .await {
            Ok(row) => row,
            Err(error) => {
                println!("{}", error);
                return Redirect::to(fallback_url)
            },
        }
        .try_get(0)
       .ok();

    let url_str = match url {
        Some(url_str) => url_str,
        None => {
            println!("failed");
            fallback_url
        },
    };

    Redirect::to(url_str)
}

#[get("/short")]
pub async fn index() -> Template {
    Template::render("urlshrtner/index", context! {})
}

#[derive(FromForm, Debug, Validate)]
pub struct Url {
    #[validate(url)]
    pub url: String
}

#[allow(unused_assignments)]
#[post("/short", data = "<form>")]
pub async fn submit(mut db: Connection<Urls>, form: Form<Url>) -> Template {
    let mut url = form.into_inner().url;

    let mut mod_url = String::new();
    let is_valid = validate_url(&url);
    if !is_valid {
        mod_url = "https://".to_string();
        mod_url.push_str(url.as_str());
        if validate_url(&mod_url) {
            url = mod_url;
        } else {
            return Template::render("urlshrtner/index", context! {
                url: "URL is not valid"
            });
        }
    }

    let url_hash = short::hash(&url);

    sqlx::query("INSERT INTO urls (url, url_hash) VALUES (?, ?)").bind(&url).bind(&url_hash)
        .execute(&mut *db)
        .await
        .ok();

    Template::render("urlshrtner/result", context! {
        url: format!("plexx.dev/{}", url_hash)
    })
}