use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginCheck};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;
use std::fs;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}
pub async fn get_login(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-login-basic", &json!({"m":"ASs"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn login(

    form: web::Form<User>,
    req: HttpRequest,
    _user: Option<Identity>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = &form.username;
    let password = &form.password.to_string();
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
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .content_type(ContentType::html())
            .finish())
    }
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    web::Redirect::to("/")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(_user) = user {
        web::Redirect::to("/admin/posts/page/1")
    } else {
        web::Redirect::to("/")
    }
}

pub async fn failed_login_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("llogin", &json!({"":""}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
