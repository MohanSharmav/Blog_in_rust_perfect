use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginCheck};
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("login", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_data_from_login_page(
    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<ConfigurationConstants>,
) -> Result<Redirect, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
println!("-------------------------------- 😂 recieved data from login page");
    let mcrypt = &config.magic_key;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    let db = &config.database_connection;
    let result = login_database(username, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

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
