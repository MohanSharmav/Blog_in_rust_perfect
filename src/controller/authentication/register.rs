use crate::controller::authentication::login::User;
use crate::model::authentication::register_database::register_new_user_database;
use actix_web::{web, HttpResponse};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde_json::json;
use std::fs;

pub async fn get_register_page() -> HttpResponse {
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/register.hbs").unwrap();
    handlebars
        .register_template_string("register", &index_template)
        .expect("TODO: panic message");

    let html = handlebars
        .render("register", &json!({"yy":"uuihiuhuihiuhuih"}))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_data_from_register_page(form: web::Form<User>) -> HttpResponse {
    let user = &form.username;
    let password = &form.password;

    let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
    let encrypted_password = mcrypt.encrypt_str_to_base64(password); //Encrypts the string and saves it to the 'encrypted_string' variable.

    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template)
        .expect("TODO: panic message");

    register_new_user_database(user, encrypted_password)
        .await
        .expect("TODO: panic message");

    let success_message = "user successfully authenticated";
    let html = handlebars
        .render("message_display", &json!({ "message": success_message }))
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
