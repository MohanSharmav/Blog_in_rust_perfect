use crate::controllers::authentication::session::User;
use crate::controllers::constants::Configuration;
use crate::model::authentication::register::register_user;
use actix_http::header::LOCATION;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use handlebars::Handlebars;
use serde_json::json;
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum PasswordError {
//     #[error("error hashing password: {0}")]
//     Hash(String),
//     #[error("error verifying password")]
//     Verify,
//     #[error("error hashing password")]
//     PwHash,
//     #[error("error getting enough random data")]
//     RandomFillError,
// }

pub async fn get_register(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-register-basic", &json!({"yy":"welcome"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn register(
    form: web::Form<User>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = &form.username;
    let password = &form.password;
    let db = &config.database_connection;
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // let password_hash = argon2.hash_password("password".as_ref(), &salt)
    //     .map_err(actix_web::error::ErrorInternalServerError)?;

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        // .map_err(|e| PasswordError::Hash(e.to_string()))?;
        .unwrap()
        // .unwrap_or_else(|_|password_hash)
        .to_string();

    register_user(user, password_hash, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
}
