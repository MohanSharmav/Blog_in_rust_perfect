use crate::controllers::constants::Configuration;
use crate::model::authentication::session::{login_database, password_check};
use crate::model::structs::LoginCheck;
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::cookie::Key;
use actix_web::http::header::ContentType;
use actix_web::{http, web, HttpResponse};
use actix_web::{HttpMessage as _, HttpRequest, Responder};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::{FlashMessage, FlashMessagesFramework, IncomingFlashMessages};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use secrecy::Secret;
use serde::Deserialize;
use serde_json::json;
use std::borrow::Borrow;
use std::fmt::Write;
use std::ptr::hash;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub async fn get_login(
    handlebars: web::Data<Handlebars<'_>>,
    flash_message: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    for message in flash_message.iter() {
        writeln!(error_html, "{}", message.content())
            .map_err(actix_web::error::ErrorInternalServerError)?;
    }
    let html = handlebars
        .render("auth-login-basic", &json!({ "message": error_html }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn login(
    form: web::Form<User>,
    req: HttpRequest,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = &form.username;
    let password = &form.password;
    let db = &config.database_connection;
    // let salt = SaltString::generate(&mut OsRng);
    // let argon2 = Argon2::default();
    // // let password_hash = argon2
    // //     .hash_password(password.as_bytes(), &salt)
    // //     .unwrap()
    // //     .to_string();

    let parsed_hash = password_check(username.clone(), db)
        .await
        .unwrap_or("asdasd".parse()?);
        // .map_err(actix_web::error::ErrorInternalServerError)?;

    let parsed_stored =
        PasswordHash::new(&*parsed_hash).map_err(actix_web::error::ErrorInternalServerError)?;

    let result = Argon2::default()
        .verify_password(password.as_bytes(), parsed_stored.borrow())
        .is_ok();

    if result == true {
        Identity::login(&req.extensions(), username.to_string())
            .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/admin/posts/page/1"))
            .finish())
    } else {
        FlashMessage::error("Login Fail - Wrong Id or password!").send();
        Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/login"))
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
pub fn build_message_framework(signing_key: Key) -> FlashMessagesFramework {
    let message_store = CookieMessageStore::builder(signing_key).build();
    FlashMessagesFramework::builder(message_store).build()
}
