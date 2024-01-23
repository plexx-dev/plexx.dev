use std::fmt;
use std::net::IpAddr;

use chrono;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{form::Form, *};
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[allow(unused_imports)]
use rocket_db_pools::{Connection, Database};
use rocket_db_pools::sqlx::{self, Row};

use validator::{Validate, validate_url};

use crate::Urls;

use super::short;

#[get("/<url_hash>")]
pub async fn get_url(mut db: Connection<Urls>, url_hash: String, remote_ip: IpAddr, frwd_ip: FrwdIP, ua: UserAgent) -> Redirect {
    let fallback_url = String::from("https://plexx.dev");
    
    let url: Option<String> = match sqlx::query("SELECT url FROM urls WHERE url_hash = ?")
        .bind(&url_hash)
        .fetch_one(&mut **db)
        .await {
            Ok(row) => row,
            Err(error) => {
                println!("{}", error);
                return Redirect::to(fallback_url)
            },
        }
        .try_get(0)
       .ok();

    let mut ip = frwd_ip.0;
    if ip.starts_with("127.0.0.1") {
        ip = remote_ip.to_string();
    }

    let url_str = match url {
        Some(url_str) => {
            sqlx::query("INSERT INTO views (url_hash, ip_address, time, useragent) VALUES (?, ?, ?, ?)").bind(&url_hash).bind(&ip).bind(format!("{:?}", chrono::offset::Local::now())).bind(&ua.0)
            .execute(&mut **db)
            .await
            .ok();

            sqlx::query("UPDATE total_views SET total_views = total_views + 1 WHERE url_hash = ?").bind(&url_hash)
            .execute(&mut **db)
            .await
            .ok();

            url_str
        },
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

#[derive(Debug)]
pub struct UserAgent(String);

#[derive(Debug)]
pub enum UserAgentError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent {
    type Error = UserAgentError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("User-Agent") {
            None => Outcome::Error((Status::BadRequest, UserAgentError::Missing)),
            Some(user_agent) => Outcome::Success(UserAgent(user_agent.to_string())),
            // if i need to validdate this :) Some(_) => Outcome::Error((Status::BadRequest, UserAgentError::Invalid)),
        }
    }
}

#[derive(Debug)]
pub struct FrwdIP(String);

#[derive(Debug)]
pub enum FrwdIPError {
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FrwdIP {
    type Error = FrwdIPError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("{:?}", &req.headers());
        match req.headers().get_one("X-Real-IP") {
            None => Outcome::Success(FrwdIP("127.0.0.1".to_string())),
            Some(user_agent) => Outcome::Success(FrwdIP(user_agent.to_string())),
            // if i need to validdate this :) Some(_) => Outcome::Error((Status::BadRequest, UserAgentError::Invalid)),
        }
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
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
        .execute(&mut **db)
        .await
        .ok();

    sqlx::query("INSERT INTO total_views (url_hash, total_views) VALUES (?, ?)").bind(&url_hash).bind(0)
        .execute(&mut **db)
        .await
        .ok();

    Template::render("urlshrtner/result", context! {
        url: format!("plexx.dev/{}", url_hash)
    })
}
