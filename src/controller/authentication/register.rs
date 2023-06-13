use crate::controller::authentication::login::User;
use crate::model::authentication::register_database::register_new_user_database;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde_json::json;
use std::fs;

pub async fn get_register_page() -> Result<HttpResponse, actix_web::Error> {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/register.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;
    handlebars
        .register_template_string("register", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render("register", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_data_from_register_page(
    form: web::Form<User>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = &form.username;
    let password = &form.password;

    let magic_key =
        std::env::var("MAGIC_KEY").map_err(actix_web::error::ErrorInternalServerError)?;

    let mcrypt = new_magic_crypt!(magic_key, 256);

    let encrypted_password = mcrypt.encrypt_str_to_base64(password); //Encrypts the string and saves it to the 'encrypted_string' variable.

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    handlebars
        .register_template_string("message_display", &index_template)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    register_new_user_database(user, encrypted_password)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let success_message = "user successfully authenticated";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
