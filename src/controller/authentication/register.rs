use crate::controller::authentication::login::User;
use crate::controller::constants::ConfigurationConstants;
use crate::model::authentication::register_database::register_new_user_database;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde_json::json;

pub async fn get_register_page(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("register", &json!({"yy":"uuihiuhuihiuhuih"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn get_data_from_register_page(
    form: web::Form<User>,
    handlebars: web::Data<Handlebars<'_>>,
    config: web::Data<ConfigurationConstants>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = &form.username;
    let password = &form.password;
    let mcrypt = &config.magic_key;
    let db = &config.database_connection;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password); //Encrypts the string and saves it to the 'encrypted_string' variable.
    register_new_user_database(user, encrypted_password, db)
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
