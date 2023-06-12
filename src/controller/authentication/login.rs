use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::fs;

use actix_identity::Identity;
use actix_web::web::Redirect;
use actix_web::{HttpMessage as _, HttpRequest, Responder};

use crate::model::authentication::login_database::{login_database, LoginCheck};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login_page() -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/login.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("login", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("login", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

pub async fn get_data_from_login_page(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
) -> Result<Redirect, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();

    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);

    let result = login_database(username, encrypted_password)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let _x = 1_i64;
    let y = LoginCheck { value: 1 };

    if result == y {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(web::Redirect::to("/admin?page=1&limit=2"))
    } else {
        Ok(web::Redirect::to("/login"))
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/?page=1")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(_user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")
    }
}
