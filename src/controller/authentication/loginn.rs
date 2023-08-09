mod get;
mod post;
use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::login_database::{login_database, LoginCheck};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use crate::controller::common::all_structs::User;

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
        .render("llogin", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
